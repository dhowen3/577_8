Coding Question: Knapsack \
Implement the algorithm for the Knapsack Problem in either C, C++, C#, Java, Python, or Rust. Be
efficient and implement it in $O(nW)$ time, where $n$ is the number of items and $W$ is the capacity. \
The input will start with an positive integer, giving the number of instances that follow. For each
instance, there will two positive integers, representing the number of items and the capacity, followed
by a list describing the items. For each item, there will be two nonnegative integers, representing the
weight and value, respectively. \
A sample input is the following: \
2 \
1 3 \
4 100 \
3 4 \
1 2 \
3 3 \
2 4 \
The sample input has two instances. The first instance has one item and a capacity of 3. The item has
weight 4 and value 100. The second instance has three items and a capacity of 4. \
For each instance, your program should output the maximum possible value. The correct output to the
sample input would be: \
0 \
6 \
