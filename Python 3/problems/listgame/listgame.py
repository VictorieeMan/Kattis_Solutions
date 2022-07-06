# Created: 2022-07-06
# Link: https://open.kattis.com/problems/listgame

import sys

# Kattis / Machine input
for i in sys.stdin:
    input = int(i)

# # Manual input
# input = 65536

### Main
ans = input
div = 2
k = 0

while ans != 1:
    ans /= div
    if int(ans) == ans:
        k += 1
    else:
        ans = int(ans*div)
        div += 1

print(k)
# print("Done!")