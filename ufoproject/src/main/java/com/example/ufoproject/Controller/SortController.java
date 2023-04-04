package com.example.ufoproject.Controller;

import com.example.ufoproject.custom.BubbleSort;
import com.example.ufoproject.custom.MergeSort;
import com.example.ufoproject.rust.RustSort;
import java.util.Arrays;
import java.util.concurrent.TimeUnit;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.CacheControl;
import org.springframework.http.ResponseEntity;
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

    public static <T> ResponseEntity<T> noValidateResponse(T data) {
        return ResponseEntity.ok()
            .cacheControl(CacheControl.maxAge(0, TimeUnit.SECONDS).mustRevalidate().cachePrivate())
            .body(data);
    }

    @GetMapping("/buildin-java-sort")
    public ResponseEntity<int[]> buildInJavaSort(@RequestBody int[] arr) {
        Arrays.sort(arr);
        return noValidateResponse(arr);
    }

    @GetMapping("/java-bubble-sort")
    public ResponseEntity<int[]> javaBubbleSort(@RequestBody int[] arr) {
        javaBubbleSort.bubbleSort(arr);
        return noValidateResponse(arr);
    }

    @GetMapping("/java-merge-sort")
    public ResponseEntity<int[]> javaMergeSort(@RequestBody int[] arr) {
        javaMergeSort.mergeSort(arr, 0, arr.length - 1);
        return noValidateResponse(arr);
    }

    @GetMapping("/rust-merge-sort")
    public ResponseEntity<int[]> rustMergeSort(@RequestBody int[] arr) {
        RustSort.mergeSort(arr);
        return noValidateResponse(arr);
    }

    @GetMapping("/rust-bubble-sort")
    public ResponseEntity<int[]> rustBubbleSort(@RequestBody int[] arr) {
        RustSort.bubbleSort(arr);
        return noValidateResponse(arr);
    }
}