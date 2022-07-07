// Created: 2022-07-07
// Link: https://open.kattis.com/problems/crypto

// Kattis allows all standard libraries included in C++
#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

int search_result(std::vector<int> vec, int x){
    std::vector<int>::iterator it;
    it = std::find(vec.begin(), vec.end(), x);
    if(it != vec.end()){
        return (it - vec.begin());
    }
    else{
        return -1;
    }
}

int main() {
    int a;
    std::cin >> a;

    std::vector<int> trail_zeroes_count; // Index 0 == Base 2
    for(int i = 2; i < a;i++){
        //Utilizing The Euclidean Algorithm
        int p = a;
        int b = i;
        int r = (p % b);
        int q = (p-r)/b;
        int k = 0;

        while(q != 0){
            if(r == 0){
                k++;
            }
            p = q;
            r = (p % b);
            q = (p-r)/b;
            if(r != 0){
                trail_zeroes_count.push_back(k);
                break;
            }
        }
    }
    int z_max = *std::max_element(trail_zeroes_count.begin(),trail_zeroes_count.end());
    // std::cout << z_max;

    int base_k = search_result(trail_zeroes_count, z_max) + 2; //Add 2 Because of how vector elements are indexed
    std::cout << base_k;

    // std::cout << "Hello World!";

    return 0;
} 