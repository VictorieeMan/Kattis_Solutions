// Created: 2022-08-25
// Link: https://open.kattis.com/problems/turbo

//Problem type: Algorithm
//Solution: 

// Kattis allows all standard libraries included in C++
#include <algorithm>
#include <iostream>
#include <vector>
#include <deque>

int main() {
    int n;
    std::cin >> n;
    std::vector<int> array;
    for(int i = 0; i < n; i++){
        int numb;
        std::cin >> numb;
        array.push_back(numb);
    }

    int phase_swaps;
    int front_key = 1;
    int back_key = n;

    std::vector<int>::iterator it;

    for(int i = 0; i < n; i++){
        if(i % 2 == 0){

            it = std::find(array.begin(),array.end(), front_key);
            ++front_key;

            array.erase(it);
            phase_swaps = it - array.begin();

        } else if(i % 2 == 1){

            it = std::find(array.begin(),array.end(), back_key);
            --back_key;

            array.erase(it);
            phase_swaps = array.size()-(it - array.begin());

        }
        std::cout << phase_swaps << '\n';
    }

    
    return 0;
}