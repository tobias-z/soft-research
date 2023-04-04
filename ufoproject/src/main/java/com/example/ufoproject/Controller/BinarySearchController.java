package com.example.ufoproject.Controller;

import com.example.ufoproject.custom.binarySearchTree.BinaryTreeCustom;
import com.example.ufoproject.rust.RustBinarySearchTree;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
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
    public ResponseEntity<SearchResponse> rustBinarySearch(@RequestBody SearchRequest searchRequest) {
        int[] indexes = RustBinarySearchTree.binarySearch(searchRequest.sortedArray(), searchRequest.numbersToFind());
        return SortController.mustRevalidateResponse(new SearchResponse(indexes));
    }

    @GetMapping("/built-in-binary-search")
    public ResponseEntity<SearchResponse> builtInBinarySearch(@RequestBody SearchRequest searchRequest) {
        int[] indexes = new int[searchRequest.numbersToFind.length];
        for (int i = 0; i < searchRequest.numbersToFind.length; i++) {
            int index = Arrays.binarySearch(searchRequest.sortedArray(), searchRequest.numbersToFind[i]);
            indexes[i] = index;
        }
        return SortController.mustRevalidateResponse(new SearchResponse(indexes));
    }

    @GetMapping("/custom-binary-search")
    public ResponseEntity<SearchResponse> customBinarySearch(@RequestBody SearchRequest searchRequest) {
        // int[] sortedArray = new int[]{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 20, 25};
        // int numberToFind = 10;
        BinaryTreeCustom.root = binaryTreeCustom.sortedArrayToBalancedTree(searchRequest.sortedArray, 0, searchRequest.sortedArray.length - 1);
        return SortController.mustRevalidateResponse(new SearchResponse(binaryTreeCustom.searchArr(BinaryTreeCustom.root, searchRequest.numbersToFind)));
    }

}
