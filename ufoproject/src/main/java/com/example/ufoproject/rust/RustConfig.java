package com.example.ufoproject.rust;

import org.springframework.context.annotation.Configuration;

@Configuration
public class RustConfig {
    static {
        // Load the implementations from our rust library
        System.loadLibrary("lib_rust");
    }
}
