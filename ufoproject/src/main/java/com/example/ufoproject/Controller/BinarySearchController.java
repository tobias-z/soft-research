package com.example.ufoproject.Controller;

import com.example.ufoproject.rust.RustBinarySearchTree;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.Arrays;
import java.util.TreeMap;
import java.util.TreeSet;

@RestController
@RequestMapping("/search")
public class BinarySearchController {

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

}
