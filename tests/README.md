## Running k6s tests

All the tests are located inside the ``k6s`` folder.

#### Environment needed:
* ``__ENV.HOME`` - The "home" folder of the user. Is used to locate the files inside `/documents`
* ``__ENV.TEST_AMOUNT`` - What test file inside `/documents` should be used.
  * The following options are currently available:
    * 100
    * 1000
    * 10000
    * 100000
    * 5000
    * 50000
    * 500000
* ``__ENV.TARGET``
  * There are two "root" endpoints:
  * Example: ``/sort/buildin-java-sort`` or ``/search/built-in-binary-search``
    * ``/sort`` - Any of the sorting endpoints **only accepts a array of integers**.
      * ``/buildin-java-sort`` - Is java's default built in easy to use sorting method of an array.
      * ``/java-bubble-sort`` - Is a custom Java implementation of sorting an array using a bubble sorting algorithm.
      * ``/java-merge-sort`` - Is a custom Java implementation of sorting an array using a merge sorting algorithm.
      * ``/rust-bubble-sort`` - Is a custom Rust implementation of the bubble sorting algorithm.
      * ``/rust-merge-sort`` - Is a custom Rust implementation of the merge sorting algorithm.
      * All of the above ``/sort`` endpoints will the array it was given but sorted **ascending** from the lowest value.
    * ``/search/`` - Any of the search endpoints **only accepts a sorted array of integers and an array of integers you are looking for**.
      * ``/built-in-binary-search`` Is Java's default built in method for searching for specific elements in an array.
      * ``/custom-binary-search`` Is a custom Java implementation using a Binary Search Tree to finding the values.
      * ``/rust-binary-search`` Is a custom Rust implementation using a Binary Search Tree to find the values.
      * All of the above ``/search`` endpoints will return an array of indexes that correlate to the number you want to find in the sorted array.

