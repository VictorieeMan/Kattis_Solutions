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