# Build the so file from our rust library
FROM rust:1.68.0-slim as rust_builder
WORKDIR /build
COPY lib-rust .
RUN rustup default nightly
RUN cargo build --release

FROM openjdk:19-bullseye as java_builder
WORKDIR /build
COPY ufoproject .
RUN ./mvnw package -Dmaven.test.skip=true

FROM openjdk:19-bullseye
RUN mkdir -p /usr/java/packages/lib
COPY --from=rust_builder /build/target/release/liblib_rust.so /usr/java/packages/lib/liblib_rust.so
COPY --from=java_builder /build/target/ufoproject*.jar /app.jar
CMD [ "java", "-jar", "/app.jar" ]
