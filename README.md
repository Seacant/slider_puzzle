# slider_puzzle

Use Depth-first search to find the solution to a slider puzzle. 

## Example Output:
```
1|2|3
4|5|6
7|8|0
Direction: DOWN
1|2|3
4|5|0
7|8|6
Direction: DOWN
1|2|0
4|5|3
7|8|6
Direction: RIGHT
1|0|2
4|5|3
7|8|6
Direction: UP
1|5|2
4|0|3
7|8|6
Direction: RIGHT
1|5|2
0|4|3
7|8|6
Direction: UP
1|5|2
7|4|3
0|8|6
Direction: LEFT
1|5|2
7|4|3
8|0|6
Direction: LEFT
1|5|2
7|4|3
8|6|0
Direction: DOWN
1|5|2
7|4|0
8|6|3
Direction: DOWN
1|5|0
7|4|2
8|6|3
Direction: RIGHT
1|0|5
7|4|2
8|6|3
Direction: UP
1|4|5
7|0|2
8|6|3
Direction: UP
1|4|5
7|6|2
8|0|3
Direction: LEFT
1|4|5
7|6|2
8|3|0
Direction: DOWN
1|4|5
7|6|0
8|3|2
Direction: RIGHT
1|4|5
7|0|6
8|3|2
Direction: RIGHT
1|4|5
0|7|6
8|3|2
Direction: UP
1|4|5
8|7|6
0|3|2
Direction: LEFT
1|4|5
8|7|6
3|0|2
Direction: LEFT
1|4|5
8|7|6
3|2|0
Direction: DOWN
1|4|5
8|7|0
3|2|6
Direction: RIGHT
1|4|5
8|0|7
3|2|6
Direction: UP
1|4|5
8|2|7
3|6|0
Found solution using 276198 moves for a pathlength of 24
```
