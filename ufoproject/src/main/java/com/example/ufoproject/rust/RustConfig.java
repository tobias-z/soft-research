package com.example.ufoproject.rust;

import jakarta.annotation.PostConstruct;
import lombok.extern.slf4j.Slf4j;
import org.springframework.context.annotation.Configuration;

@Configuration
@Slf4j
public class RustConfig {

    @PostConstruct
    private void postConstruct() {
        try {
            // Load the implementations from our rust library
            System.loadLibrary("lib_rust");
        } catch (Throwable e) {
            log.error("unable to load the native rust library. Java could not find it in the paths that your OS requires it", e);
            log.info("to make it work follow the guide written in the Native Integration section of the README.md");
        }
    }

}
