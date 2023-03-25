package com.example.ufoproject.Controller;

import com.example.ufoproject.custom.BubbleSort;
import com.example.ufoproject.custom.MergeSort;
import com.example.ufoproject.rust.RustSort;
import java.util.Arrays;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/sort")
public class SortController {

    @Autowired
    private BubbleSort javaBubbleSort;
    @Autowired
    private MergeSort javaMergeSort;

    @GetMapping("/buildin-java-sort")
    public int[] buildInJavaSort(@RequestBody int[] arr) {
        Arrays.sort(arr);
        return arr;
    }

    @GetMapping("/java-bubble-sort")
    public int[] javaBubbleSort(@RequestBody int[] arr) {
        javaBubbleSort.bubbleSort(arr);
        return arr;
    }

    @GetMapping("/java-merge-sort")
    public int[] javaMergeSort(@RequestBody int[] arr) {
        javaMergeSort.mergeSort(arr, 0, arr.length - 1);
        return arr;
    }

    @GetMapping("/rust-merge-sort")
    public int[] rustMergeSort(@RequestBody int[] arr) {
        RustSort.mergeSort(arr);
        return arr;
    }

    @GetMapping("/rust-bubble-sort")
    public int[] rustBubbleSort(@RequestBody int[] arr) {
        RustSort.bubbleSort(arr);
        return arr;
    }
}