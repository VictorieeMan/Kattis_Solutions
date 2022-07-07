// Kattis allows all standard libraries included in C++
#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

// Created: 2022-07-07
// Link: https://open.kattis.com/problems/crypto
// Not used in solution.
std::vector<int> base_conversion(int p, int b){
    std::vector<int> in_new_base;
    int r = (p % b);
    int q = (p-r)/b;

    while(q != 0){
        in_new_base.push_back(r);
        p = q;
        r = (p % b);
        q = (p-r)/b;
    }
    
    in_new_base.push_back(r);

    return in_new_base;
}