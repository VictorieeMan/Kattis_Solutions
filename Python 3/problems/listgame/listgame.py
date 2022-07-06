# Created: 2022-07-06
# Link: https://open.kattis.com/problems/listgame

import sys

# # Kattis / Machine input
# for i in sys.stdin:
#     input = int(i)

# Manual input
input = 127381

# Functions
def check_coPrime(a,b):
    isCoprime = True
    minAB = min(a,b)

    if minAB < 2:
        isCoprime = False
    else:
        for i in range(2,minAB+1):
            if (a % i == 0 and b % i == 0):
                isCoprime = False
                break
    
    return isCoprime

# def keep_coPrime(numb, list):
#     listOfCoprime = [numb]
#     for i in list:
#         if check_coPrime(numb,i):
#             listOfCoprime.append(i)
#     return listOfCoprime


### Main
cop_factors = []
for i in range(2,input):
    if (input % i == 0):
        if len(cop_factors) == 0:
            cop_factors.append(i)
        else:
            for j in cop_factors:
                if check_coPrime(i, j):
                    cop_factors.append(i)

print(cop_factors)

# print(keep_coPrime(cop_factors[0],cop_factors))

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
print("Done!")