// Created: 2022-08-25
// Link: https://open.kattis.com/problems/turbo

//Problem type:
//Solution: 

// Kattis allows all standard libraries included in C++
#include <iostream>
#include <vector>
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

std::deque<int> swap_stepping(std::deque<int> list, int a, int b){
    // Moves element from 
}

int main() {
    int n;
    std::cin >> n;
    std::vector<std::pair<int,int>> array;
    for(int i = 0; i < n; i++){
        int numb;
        std::cin >> numb;
        array.push_back(std::pair(i,numb));
    }
    
    return 0;
}