# Research Project

## Group Members

- Mads Bryde, cph-mb870@cphbusiness.dk
- Mathias Egebjerg Jensen, cph-mj839@cphbusiness.dk
- Malthe Stefan Woschek, cph-mw202@cphbusiness.dk
- Tobias Zimmermann, cph-tz11@cphbusiness.dk

## Native Integration

Running without having the native rust library installed works fine as long as you are not trying to call them.

To actually run them, you have two options:

1. (The easiest) From the root of this repository run `docker compose up` this will start the application on port 8080 with the rust library built, and you are able to call them.
2. Build it yourself:
  - if you do not have rust installed, follow this link to install it https://www.rust-lang.org/tools/install
  - run `cargo build` inside the `lib-rust` directory
  - depending on the operating system you are using, the generated file has to be put somewhere on the system that Java is telling you about. This path is written to you when you try to start the app without having the library. The generated file (on Linux this is called .so. Not sure about mac and windows) located in the `target/debug` folder has to be put inside either of the directories told you by the spring boot app when starting it.
  - You can now start the app without docker and run the native functions
