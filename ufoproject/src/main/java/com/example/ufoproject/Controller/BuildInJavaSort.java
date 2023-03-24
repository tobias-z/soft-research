package com.example.ufoproject.Controller;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.Arrays;

@RestController
@RequestMapping
public class BuildInJavaSort {
    @GetMapping("/BuildInJavaSort")
    public int[] buildInJavaSort(int[] arr) {
        Arrays.sort(arr);
        return arr;
    }
}