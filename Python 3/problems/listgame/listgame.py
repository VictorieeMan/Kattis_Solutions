# Created: 2022-07-06
# Link: https://open.kattis.com/problems/listgame

import sys

from math import sqrt as sqrt

# Kattis / Machine input
input = sys.stdin.read()

# # Manual input
# input = 127381123

### Main
number = int(input.split("\n")[0])
maxCheck = sqrt(number) + 1
ans = number
div = 2
k = 0

while ans != 1:
    temp_ans = ans / div
    if int(temp_ans) == temp_ans: # Check if integer
        ans = temp_ans
        k += 1
    else:
        div += 1
        if div >= maxCheck:
            k = 1
            break

print(k)
# print("Done!")