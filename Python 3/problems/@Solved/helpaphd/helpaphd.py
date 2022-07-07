# Created: 2022-07-06
# Link: https://open.kattis.com/problems/helpaphd

#Recommended from the guide: https://github.com/VictorieeMan/kattis-guide/blob/master/input.md

import sys

# Kattis / Machine input
input = sys.stdin.read()

# Manual input
# input = "4\n\2+2\1+2\p=NP\0+0"

string = input.split("\n")[1:-1]

for i in string:
    if i == "P=NP":
        print("skipped")
    else:
        add = i.split("+")
        print(int(add[0])+int(add[1]))

print("Done!")