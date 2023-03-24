package com.example.ufoproject.custom;

import org.springframework.stereotype.Component;

//https://howtodoinjava.com/algorithm/merge-sort-java-example/
@Component
public class MergeSort {
    /**
     * name MergeSort
     * */
    public void mergeSort(int[] arr, int low, int high) {
        if(low < high) {
            int mid = (low + high) / 2;
            mergeSort(arr, low, mid);
            mergeSort(arr, (mid + 1), high);
            merge(arr, low, mid, high);
        }
    }

    private void merge(int[] arr, int low, int mid, int high) {
        int lowMid = mid - low + 1;
        int midHigh = high - mid;

        int[] _LOW = new int[lowMid];
        int[] _HIGH = new int[midHigh];

        for(int i = 0; i < lowMid; i++) {
            _LOW[i] = arr[lowMid + i];
        }

        for(int j = 0; j < midHigh; j++) {
            _HIGH[j] = arr[mid + 1 + j];
        }

        int i = 0, j = 0, k = low;

        while (i < lowMid && j < midHigh) {
            if(_LOW[i] <= _HIGH[j]) {
                arr[k] = _LOW[i];
                i++;
            } else {
                arr[k] = _HIGH[j];
                j++;
            }
            k++;
        }

        while(i < lowMid) {
            arr[k] = _LOW[i];
            k++;
            i++;
        }

        while (j < midHigh) {
            arr[k] = _HIGH[j];
            k++;
            j++;
        }
    }
}
