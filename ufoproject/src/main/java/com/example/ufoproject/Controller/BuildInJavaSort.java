package com.example.ufoproject.Controller;

import com.example.ufoproject.custom.BubbleSort;
import com.example.ufoproject.custom.MergeSort;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

import java.util.Arrays;
import java.util.Random;

@RestController
@RequestMapping
public class BuildInJavaSort {
    @Autowired BubbleSort bubbleSort;
    @Autowired MergeSort mergeSort;

    @GetMapping("/BuildInJavaSort")
    public int[] buildInJavaSort(int[] arr) {
        Arrays.sort(arr);
        return arr;
    }

    @GetMapping("/BubbleSort")
    public int[] bubbleSortEndpoint(int[] arr) {
        bubbleSort.bubbleSort(arr);
        return new int[0];
    }

    @GetMapping("/MergeSort")
    public int[] mergeSortEndpoint() {
        int[] arr = new int[10];
        for (int i = 0; i < 10; i++) {
            arr[i] = new Random().nextInt();
        }
        mergeSort.mergeSort(arr, 0, arr.length - 1);
        System.out.println(Arrays.toString(arr));
        return new int[0];
    }
}