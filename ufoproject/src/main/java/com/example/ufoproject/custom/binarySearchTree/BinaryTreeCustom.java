package com.example.ufoproject.custom.binarySearchTree;

import org.springframework.stereotype.Component;

@Component
public class BinaryTreeCustom {

    public static Node root;

    /* Constructing the Balanced Binary Tree from a sorted array. */
    public Node sortedArrayToBalancedTree(int[] arr, int start, int end) {

        /* Base Case */
        if (start > end) {
            return null;
        }

        /* Get the middle element and make it the root node */
        int mid = (start + end) / 2;
        Node node = new Node(arr[mid], mid);

        /* Recursively insert the left subtree/node and make it left child of the root node */
        node.left = sortedArrayToBalancedTree(arr, start, mid - 1);

        /* Recursively insert the right subtree/node and make it right child of the root node */
        node.right = sortedArrayToBalancedTree(arr, mid + 1, end);

        return node;
    }

    /* This should maybe return the node itself instead of a number. Because if we are actually searching for "-1337" we don't know if it's actually in the tree or not. */
    public int search(Node root, int data) {
        if (root == null) {
            return -1;
        } else if (root.data == data) {
            return root.index;
        } else if (root.data > data) {
            return search(root.left, data);
        }
        return search(root.right, data);
    }

    public int[] searchArr(Node root, int[] searchFor) {
        int[] index = new int[searchFor.length];

        for(int i = 0; i < searchFor.length; i++) {
            index[i] = search(root, searchFor[i]);
        }

        return index;
    }

    /* Print the preorder traversal of the balanced tree */
    public void preOrder(Node node) {
        if (node == null) {
            return;
        }
        System.out.print(node.data + " ");
        preOrder(node.left);
        preOrder(node.right);
    }
}