// Created: 2022-08-25
// Link: https://open.kattis.com/problems/turbo

//Problem type:
//Solution: 

// Kattis allows all standard libraries included in C++
#include <iostream>
#include <deque>

int find_index(int key, std::deque<int> list){
    //Assumes that key actually exist in the list.
    int size = list.size();
    for(int i = 0; i < size; i++){
        if(key == list[i]){
            return i;
        }
    }
    return size;
}

// std::deque<int> swap_stepping(std::deque<int> list, int a, int b){
//     // Moves element from 
// }

int main() {
    int n;
    std::cin >> n;
    std::deque<int> array(n);
    for(int i = 0; i < n; i++){
        std::cin >> array[i];
    }

    std::deque<int> phases(n);
    for(int i = 0; i < n; i++){
        phases[i] = i+1;
    }

    for(int i = 0; i < n; i++){
        if(i % 2 == 0){
            int key = phases.front();
            phases.pop_front();
            int idx = find_index(key, array);
            array.erase(array.begin()+idx);
            std::cout << idx << std::endl;
        } else if(i % 2 == 1){
            int key = phases.back();
            phases.pop_back();
            int idx = find_index(key, array);
            array.erase(array.begin()+idx);
            std::cout << array.size()-idx << std::endl;
        }
    }
    
    return 0;
}