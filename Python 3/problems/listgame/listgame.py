# Created: 2022-07-06
# Link: https://open.kattis.com/problems/listgame

import sys

from math import sqrt as sqrt

# Kattis / Machine input
for i in sys.stdin:
    input = int(i)

# # Manual input
# input = 127381123

### Main
maxCheck = sqrt(input) + 1
ans = input
div = 2
k = 0

while ans != 1:
    temp_ans = ans / div
    if int(temp_ans) == temp_ans: # Check if integer
        ans = temp_ans
        k += 1
    elif div >= maxCheck:
        k = 1
        break
    else:
        div += 1

print(k)
# print("Done!")