package com.groupawesome.example;

import com.groupawesome.example.rustlib.Sort;
import java.time.Duration;
import java.time.Instant;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.Random;

public class Main {

    static {
        // Load the implementations from our rust library
        System.loadLibrary("lib_rust");
    }

    public static void main(String[] args) {
        System.out.println(time(Main::sortWithRustMergeSort));
        System.out.println(time(Main::sortWithRustMergeSortPointerArithmetic));
//        System.out.println(time(Main::sortWithRustBubbleSort));
//        System.out.println(time(Main::sortWithJavaBubbleSort));
//        System.out.println(time(Main::sortWithBuiltIn));
    }

    private static void bubbleSort(int[] arr) {
        for (int i = 0; i < arr.length; i++) {
            for (int j = 0; j < arr.length - 1; j++) {
                int prev = arr[j];
                if (prev > arr[j + 1]) {
                    arr[j] = arr[j + 1];
                    arr[j + 1] = prev;
                }
            }
        }
    }

    private static void sortWithRustMergeSort() {
        int[] arr = new int[] {10, 0, 100, 0};
        Sort.mergeSort(arr);
        System.out.println(Arrays.toString(arr));
    }

    private static void sortWithRustMergeSortPointerArithmetic() {
        int[] arr = new int[] {10, 0, 100, 0};
        Sort.mergeSortWithArithmatic(arr);
        System.out.println(Arrays.toString(arr));
    }

    private static void sortWithRustBubbleSort() {
        int[] arr = getArrOfSize(50_000);
        Sort.bubbleSort(arr);
    }

    private static void sortWithBuiltIn() {
        int[] arr = getArrOfSize(1_000_000);
        Arrays.sort(arr);
    }

    private static void sortWithJavaBubbleSort() {
        int[] arr = getArrOfSize(50_000);
        bubbleSort(arr);
    }

    private static int[] getArrOfSize(int size) {
        int[] arr = new int[size];
        Random random = new Random();
        for (int i = 0; i < arr.length; i++) {
            arr[i] = random.nextInt();
        }
        return arr;
    }

    private static long time(VoidFn fn) {
        Instant start = Instant.now();
        fn.call();
        Instant end = Instant.now();
        return Duration.between(start, end).toMillis();
    }

    interface VoidFn {
        void call();
    }
}
