package com.example.ufoproject;

import com.example.ufoproject.rust.RustSort;
import java.time.Duration;
import java.time.Instant;
import java.util.Arrays;
import java.util.Random;

public class Main {

    static {
        // Load the implementations from our rust library
        System.loadLibrary("lib_rust");
    }

    public static void main(String[] args) {
        System.out.printf("%s: %d%n", "Rust Bubble Sort", time(Main::sortWithRustBubbleSort));
        System.out.printf("%s: %d%n", "Java Bubble Sort", time(Main::sortWithJavaBubbleSort));
        System.out.printf("%s: %d%n", "Java Builtin", time(Main::sortWithBuiltIn));
        System.out.printf("%s: %d%n", "Rust Merge Sort", time(Main::sortWithRustMergeSort));
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
        int[] arr = getArrOfSize(1_000_000);
        RustSort.mergeSort(arr);
    }

    private static void sortWithRustBubbleSort() {
        int[] arr = getArrOfSize(50_000);
        RustSort.bubbleSort(arr);
    }

    private static void sortWithBuiltIn() {
        int[] arr = getArrOfSize(1000000);
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
