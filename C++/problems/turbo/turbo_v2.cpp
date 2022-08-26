// Created: 2022-08-25
// Link: https://open.kattis.com/problems/turbo

//Problem type:
//Solution: 

// Kattis allows all standard libraries included in C++
#include <algorithm>
#include <iostream>
#include <vector>
#include <deque>

int find_index(int key, std::vector<int> list){
    //Assumes that key actually exist in the list.
    int size = list.size();
    for(int i = 0; i < size; i++){
        if(key == list[i]){
            return i;
        }
    }
    return size;
}

int main() {
    int n;
    std::cin >> n;
    std::vector<int> array;
    for(int i = 0; i < n; i++){
        int numb;
        std::cin >> numb;
        array.push_back(numb);
    }

    std::vector<int> phase(n);
    for(int i = 0; i < n; i++){
        phase[i] = i+1;
    }
    
    int phases = n-1; // Max index of the phases array.
    int front_idx = 0;
    int back_idx = 0;
    int phase_swaps;

    std::vector<int>::iterator it;

    for(int i = 0; i < n; i++){
        if(i % 2 == 0){
            int key = phase[front_idx];

            // int idx = find_index(key, array);
            it = std::find(array.begin(),array.end(), key);
            array.erase(it);
            phase_swaps = it - array.begin();

            ++front_idx;

        } else if(i % 2 == 1){
            int key = phase[phases-back_idx];

            // int idx = find_index(key, array);
            it = std::find(array.begin(),array.end(), key);
            array.erase(it);
            phase_swaps = array.size()-(it - array.begin());

            ++back_idx;
        }
        std::cout << phase_swaps << '\n';
    }

    
    return 0;
}