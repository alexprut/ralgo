<br />
<br />

<div align="center">
<img src="https://github.com/alexprut/ralgo/raw/master/src/main/resources/logo.png" width="400" height="auto"/>

<br />
<br />

<h3>Classic Algorithms and Data Structures implemented in Rust</h3>

[![MIT](https://img.shields.io/dub/l/vibe-d.svg)](https://github.com/alexprut/ralgo/blob/master/LICENSE)


</div>
<br />

Algorithms
==========

|Algorithm|Worst-case time complexity|Average-case time complexity|Best-case time complexity|Auxiliary space complexity|
|:---|:---|:---|:---|:---|
|[Insertion Sort](https://github.com/alexprut/ralgo/algorithms/sorting/insertion_sort.rs)|Θ(n^2)|Θ(n^2)|O(n)|-|
|[BubbleSort](https://github.com/alexprut/ralgo/algorithms/sorting/bubble_sort.rs)|O(n^2)|O(n^2)|O(n)|-|
|[MergeSort](https://github.com/alexprut/ralgo/algorithms/sorting/merge_sort.rs)|Θ(nlogn)|Θ(nlogn)|Θ(nlogn)|-|
|[HeapSort](https://github.com/alexprut/ralgo/datastructures/max_heap.rs#L82)|Θ(nlogn)|-|-|-|
|[QuickSort](https://github.com/alexprut/ralgo/algorithms/sorting/quick_sort.rs)|Θ(n^2)|Θ(nlogn)|Θ(nlogn)|-|
|[CountingSort](https://github.com/alexprut/ralgo/algorithms/sorting/counting_sort.rs)|Θ(k + n)|Θ(k + n)|Θ(n)|Θ(n)|
|[RadixSort](https://github.com/alexprut/ralgo/algorithms/sorting/radix_sort.rs)|Θ(d(k + n))|Θ(d(k + n))|Θ(n)|-|
|[Floyd-Warshall](https://github.com/alexprut/ralgo/algorithms/graph/floyd_warshall.rs)|Θ(V^3)|Θ(V^3)|Θ(V^3)|Θ(V^2)|
|[Kruskal](https://github.com/alexprut/ralgo/algorithms/graph/mst/kruskal.rs)|O(\|E\|log\|V\|)|-|-|-|
|[Prim](https://github.com/alexprut/ralgo/algorithms/graph/mst/prim.rs)|O(\|E\|log\|V\|)|-|-|-|
|[Bellman–Ford](https://github.com/alexprut/ralgo/algorithms/graph/bellman_ford.rs)|Θ(\|E\|\|V\|)|-|-|Θ(V)|
|[Dijkstra](https://github.com/alexprut/ralgo/algorithms/graph/dijkstra.rs)|O(\|E\| + \|V\|log\|V\|)|-|-|-|
|[Maximum SubArray](https://github.com/alexprut/ralgo/algorithms/maximum_sub_array.rs)|Θ(n)|Θ(n)|Θ(n)|Θ(1)|
|[Knuth-Morris-Pratt](https://github.com/alexprut/ralgo/algorithms/knuth_morris_pratt.rs)|Θ(n + m)|Θ(n)|Θ(n)|Θ(n)|
|[Rabin-Karp](https://github.com/alexprut/ralgo/algorithms/rabin_karp.rs)|Θ((n - m + 1)m)|Θ(n)|Θ(n)|-|
|[Greatest common divisor (GCD)](https://github.com/alexprut/ralgo/algorithms/math/math.rs#L13)|-|-|-|-|
|[Binary Search](https://github.com/alexprut/ralgo/algorithms/search/binary_search.rs)|O(nlogn)|O(nlogn)|O(1)|Θ(1)|
|[Breadth First Search (BFS)](https://github.com/alexprut/ralgo/algorithms/search/breadth_first_search.rs#L43)|O(\|E\|+\|V\|)|O(\|E\|+\|V\|)|-|-|
|[Depth First Search (DFS)](https://github.com/alexprut/ralgo/algorithms/search/depth_first_search.rs#L10)|O(\|E\|+\|V\|)|O(\|E\|+\|V\|)|-|-|
|[Topological Sort (DFS)](https://github.com/alexprut/ralgo/algorithms/search/depth_first_search.rs#L91)|O(\|E\|+\|V\|)|O(\|E\|+\|V\|)|-|-|
|[Longest Increasing Subsequence (LIS)](https://github.com/alexprut/ralgo/utils.rs#L98)|Θ(n^2)|-|-|Θ(n)|
|[Longest Common Subsequence (LCS)](https://github.com/alexprut/ralgo/utils.rs#L128)|Θ(n^2)|-|-|Θ(n^2)|

Data Structures
===============
|Data Structure|Methods|
|--------------|-------|
|[Max Heap](https://github.com/alexprut/ralgo/datastructures/max_heap.rs)|```max()``` - Θ(1), ```extractMax()``` - O(nlogn), ```increaseKey()``` - O(logn), ```insert()``` - O(logn), ```heapify()``` - O(logn), ```heapsort()``` - O(nlogn)|
|[Min Heap](https://github.com/alexprut/ralgo/datastructures/min_heap.rs)|```min()``` - Θ(1), ```extractMin()``` - O(nlogn), ```insert()``` - O(logn), ```heapify()``` - O(logn), ```heapsort()``` - O(nlogn)|
|[MinMax Heap](https://github.com/alexprut/ralgo/datastructures/min_max_heap.rs)|```min()``` - Θ(1), ```max()``` - Θ(1), ```extractMin()``` - O(nlogn), ```extractMax()``` - O(nlogn), ```insert()``` - O(logn), ```heapify()``` - O(logn)|
|[Disjoint Set](https://github.com/alexprut/ralgo/datastructures/disjoint_set.rs)|```makeSet()``` - Θ(1), ```findSet()``` - Θ(1), ```union()``` - Θ(1)|
|[Trie](https://github.com/alexprut/ralgo/datastructures/trie.rs)|```insert()``` - O(\|s\|), ```search()``` - O(\|s\|), ```searchPrefix()``` - O(\|s\|), ```remove()``` - O(\|s\|), ```size()``` - O(1)|
|[Stack](https://github.com/alexprut/ralgo/datastructures/stack.rs)|```push()``` - Θ(1), ```pop()``` - Θ(1), ```empty()``` - Θ(1), ```size()``` - Θ(1), ```peek()``` - Θ(1)|
|[Queue](https://github.com/alexprut/ralgo/datastructures/queue.rs)|```enqueue()``` - Θ(1), ```dequeue()``` - Θ(1), ```empty()``` - Θ(1), ```size()``` - Θ(1)|
|[Binary Search Tree](https://github.com/alexprut/ralgo/datastructures/binary_search_tree.rs)|```insert()``` - O(n), ```search()``` - O(n), ```delete()``` - O(n), ```contains()``` - O(n), ```minimum()``` - O(n), ```maximum()``` - O(n), ```size()``` - Θ(1), ```successor()``` - O(n), ```preOrderVisit()``` - O(n), ```inOrderVisit()``` - O(n), ```postOrderVisit()``` - O(n)|
|[Double Linked List](https://github.com/alexprut/ralgo/datastructures/double_linked_list.rs)|```insertFront()``` - Θ(1), ```removeFront()``` - Θ(1), ```insertBack()``` - Θ(1), ```removeBack()``` - Θ(1), ```head()``` - Θ(1), ```size()``` - Θ(1)|
|[Linked List](https://github.com/alexprut/ralgo/datastructures/linked_list.rs)|```insertFront()``` - Θ(1), ```removeFront()``` - Θ(1), ```head()``` - Θ(1), ```size()``` - Θ(1)|
|[Graph](https://github.com/alexprut/ralgo/datastructures/graph.rs)|```buildAdjacencyMatrix()``` - Θ(\|V\|^2), ```buildAdjacencyList()``` - Θ(\|V\| + \|E\|), ```addEdge()``` - Θ(1)|
|[Red-Black Tree](https://github.com/alexprut/ralgo/datastructures/red_black_tree.rs)|```insert()``` - O(logn), ```search()``` - O(logn), ```delete()``` - O(logn), ```minimum()``` - O(logn), ```maximum()``` - O(logn), ```successor()``` - O(logn)|
|[Interval Tree](https://github.com/alexprut/ralgo/datastructures/interval_tree.rs)|```insert()``` - O(logn), ```search()``` - O(logn), ```find()``` - O(logn), ```findAll()``` - O(n), ```delete()``` - O(logn), ```minimum()``` - O(logn), ```maximum()``` - O(logn), ```successor()``` - O(logn)|
|[Segment Tree](https://github.com/alexprut/ralgo/datastructures/segment_tree.rs)|```build()``` - O(n), ```update()``` - O(logn), ```search()``` - O(logn)|
|[AVL Tree](https://github.com/alexprut/ralgo/datastructures/avl_tree.rs)|```insert()``` - O(logn), ```search()``` - O(logn), ```delete()``` - O(logn), ```minimum()``` - O(logn), ```maximum()``` - O(logn), ```successor()``` - O(logn)|
|[B-Tree](https://github.com/alexprut/ralgo/datastructures/b_tree.rs)|```insert()``` - O(th), ```search()``` - O(th), ```delete()``` - O(th), ```successor()``` - O(th), ```predecessor()``` - O(th)|
|[Fibonacci Heap](https://github.com/alexprut/ralgo/datastructures/fibonacci_heap.rs)|```insert()``` - O(1), ```minimum()``` - O(1), ```extractMin()``` - O(logn), ```decreaseKey()``` - O(1), ```delete()``` - O(logn)|
|[Merkle Tree](https://github.com/alexprut/ralgo/datastructures/merkle_tree.rs)|```build()``` - O(n), ```verify()``` - O(logn), ```getProofPath()``` - O(logn)|

---


### Build
```
cargo build
```

### Test
```
cargo test
```

### Formatting style
```
cargo fmt
```

### Generate Changelog

```
git-chglog v1.0.0..v2.0.0
```

License
=======
Licensed under [MIT](https://github.com/alexprut/ralgo/blob/master/LICENSE).