# Created: 2022-07-06
# Link: https://open.kattis.com/problems/listgame

import sys
from math import gcd as gcd
from math import sqrt as sqrt

# # Kattis / Machine input
# for i in sys.stdin:
#     input = int(i)

# Manual input
input = 65536

# Functions
def check_coPrime(a,b):
    isCoprime = (gcd(a,b)==1)
    
    return isCoprime

### Main
cop_factors = []
for i in range(2,int(sqrt(input))):
    if (input % i == 0):
        if len(cop_factors) == 0:
            cop_factors.append(i)
        else:
            for j in cop_factors:
                if check_coPrime(i, j):
                    cop_factors.append(i)

# print(cop_factors)

ans = input
k = 1

while ans > 1:
    for div in cop_factors:
        while True:
            ans = ans / div
            if int(ans) == ans and ans != 1:
                k += 1
                continue
            elif (int(ans) != ans):
                ans = ans * div
                break
            else:
                break

print(k)
# print("Done!")