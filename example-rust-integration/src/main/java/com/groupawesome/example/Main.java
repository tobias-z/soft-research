package com.groupawesome.example;

import com.groupawesome.example.rustlib.Sort;
import java.util.Arrays;

public class Main {

    static {
        // Load the implementations from our rust library
        System.loadLibrary("lib_rust");
    }

    public static void main(String[] args) {
        sortWithBubbleSort();
        sortWithMergeSort();
    }

    private static void sortWithMergeSort() {
        int[] arr = {1, 2, 4, 2, 1};
        Sort.mergeSort(arr);
        System.out.println(Arrays.toString(arr));
    }

    private static void sortWithBubbleSort() {
        int[] arr = {1, 2, 4, 2, 1};
        Sort.bubbleSort(arr);
        System.out.println(Arrays.toString(arr));
    }
}
