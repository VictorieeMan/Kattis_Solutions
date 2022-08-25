// Created: 2022-08-25
// Link: https://open.kattis.com/problems/stringmatching

//Problem type:
//Solution: 

// Kattis allows all standard libraries included in C++
#include <algorithm>
#include <iostream>
#include <iomanip>
#include <string>
#include <vector>
#include <cmath>
#include <tuple>

std::vector<int> search_for_matches(std::string pattern, std::string text){
    //Search for pattern matches, index them and return the list.
    std::vector<int> matches_idx;
    
    return matches_idx;
}

void output_idx_pattern_matches(std::vector<int> match_idx){
    //Outputs the indexes from the list.
    int list_size = match_idx.size();
    for(int i = 0; i < list_size; i++){
        std::cout << match_idx[i] << " ";
    }
}

int main() {
    std::string pattern;
    std::string text;
    while(std::cin >> pattern >> text){
        std::vector<int> match_idx = search_for_matches(pattern, text);
        output_idx_pattern_matches(match_idx);
        std::cout << std::endl;
    }

    return 0;
}