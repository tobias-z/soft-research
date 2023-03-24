package com.example.ufoproject.custom;

import org.springframework.stereotype.Component;

@Component
public class BubbleSort {
    public void bubbleSort(int[] arr) {
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
}
