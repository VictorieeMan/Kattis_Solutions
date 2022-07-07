// Created: 2022-07-07
// Link: https://open.kattis.com/problems/crypto

// Kattis allows all standard libraries included in C++
#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

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

int main() {
    std::cout << "Hello World! 1";
    // Standard input, alter as needed
    long long a;
    std::cin >> a;
    std::cout << "Hello World! 2";

    std::vector<int> in_new_base = base_conversion(a,2);
    for(int i = 0; i < in_new_base.size();i++){
        std::cout << in_new_base[i];
    }

    // std::cout << "Hello World!";

    return 0;
}