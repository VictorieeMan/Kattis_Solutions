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
    std::size_t text_len = text.length();
    // std::size_t pattern_len = pattern.length();
    std::size_t found = text.find(pattern);
    for(std::size_t i = 0; i < text_len; i++){
        if(found != std::string::npos){
            matches_idx.push_back(found);
        } else {
            break;
        }

        found = text.find(pattern, found+1);
    }
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
    std::vector<std::string> input;
    std::string input_string;
    while(std::getline(std::cin,input_string)){
        input.push_back(input_string);
    }
    int input_size = input.size();
    std::string pattern;
    std::string text;
    for(int i=0;i<input_size;){
        pattern = input[i];
        text = input[i+1];

        std::vector<int> match_idx = search_for_matches(pattern, text);
        output_idx_pattern_matches(match_idx);
        std::cout << std::endl;

        i += 2;
    }

    return 0;
}