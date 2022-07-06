# Created: 2022-07-06
# Link: https://open.kattis.com/problems/oddecho

# Problem needed to solve involves understanding the Kattis input system. That's where I think the runtime error comes from.

import sys

# # Kattis / Machine input
# for i in sys.stdin:
#     input = int(i)

# Manual input
input = "5\nhello\ni\nam\nan\necho"

lineArray = input.split("\n")
count = 0

for i in range(int(lineArray[0])+1):
    if count % 2 == 1:
        print(lineArray[i])
    count += 1

print("Done!")