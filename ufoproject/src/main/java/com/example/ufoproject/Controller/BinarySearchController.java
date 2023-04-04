package com.example.ufoproject.Controller;

import com.example.ufoproject.custom.binarySearchTree.BinaryTreeCustom;
import com.example.ufoproject.rust.RustBinarySearchTree;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;

import java.util.Arrays;

@RestController
@RequestMapping("/search")
public class BinarySearchController {
    @Autowired
    BinaryTreeCustom binaryTreeCustom;

    record SearchRequest(int[] sortedArray, int[] numbersToFind) {

    }

    /**
     * @param foundIndexes each index responds directly to the index of numbersToFind in the request. Meaning if the first element
     *                     in the numbersToFind array was 5, then the first element in the foundIndexes would be the result of the
     *                     search for 5.
     *                     A searched number which is resulted in not found, will be represented as -1
     */
    record SearchResponse(int[] foundIndexes) {

    }

    @GetMapping("/rust-binary-search")
    public SearchResponse rustBinarySearch(@RequestBody SearchRequest searchRequest) {
        int[] indexes = RustBinarySearchTree.binarySearch(searchRequest.sortedArray(), searchRequest.numbersToFind());
        return new SearchResponse(indexes);
    }

    @GetMapping("/built-in-binary-search")
    public int builtInBinarySearch(int[] sortedArray, int numberToFind) {
        int index = Arrays.binarySearch(sortedArray, numberToFind);
        return index;
    }

    @GetMapping("/custom-binary-search")
    public int[] customBinarySearch(@RequestBody SearchRequest searchRequest) {
        // int[] sortedArray = new int[]{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 20, 25};
        // int numberToFind = 10;
        BinaryTreeCustom.root = binaryTreeCustom.sortedArrayToBalancedTree(searchRequest.sortedArray, 0, searchRequest.sortedArray.length - 1);
        int[] index = binaryTreeCustom.searchArr(BinaryTreeCustom.root, searchRequest.numbersToFind);
        return index;
    }

}
