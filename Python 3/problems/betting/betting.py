# Created: 2022-07-06
# https://open.kattis.com/problems/betting

input = int(input("Percentage of switchpoints on option 1: "))

op1 = 100/input
op2 = 100/(100-input)

print(op1,op2)