# Challenge 364 - Easy
## Create a Dice Roller

[link](https://old.reddit.com/r/dailyprogrammer/comments/8s0cy1/20180618_challenge_364_easy_create_a_dice_roller/)

### Verification

Build:

```console
$ cargo build --release
```

Run:
```console
$ cat inputs/challenge_input.txt
5d12
6d4
1d2
1d8
3d6
4d20
100d100

$ ./target/release/c364_easy
25: [1, 10, 8, 4, 2]
12: [0, 3, 2, 2, 3, 2]
1: [1]
1: [1]
8: [2, 4, 2]
24: [3, 0, 4, 17]
4590: [53, 47, 14, 74, 12, 16, 86, 20, 24, 55, 91, 46, 2, 25, 50, 26, 65, 85, 43, 29, 51, 72, 59, 42, 17, 28, 41, 48, 78, 0, 61, 41, 69, 38, 73, 32, 24, 40, 1, 5, 74, 0, 70, 0, 82, 0, 45, 8, 63, 53, 92, 13, 44, 2, 88, 75, 12, 83, 30, 17, 96, 92, 9, 64, 58, 48, 76, 73, 52, 89, 11, 62, 6, 39, 75, 77, 91, 0, 21, 67, 29, 47, 3, 51, 89, 54, 87, 44, 87, 8, 26, 77, 85, 61, 27, 66, 32, 58, 9, 10]
```
