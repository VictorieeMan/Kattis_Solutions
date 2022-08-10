// Created: 2022-08-10
// Link: https://open.kattis.com/problems/freckles

// Kattis allows all standard libraries included in C++
// #include <algorithm>
#include <iostream>
// #include <string>
#include <vector>

int main() {
    // N is the number of test cases
    // n is the number of freckles
    // x, y are the coordinates of the freckles

    //input & process loop
    int N;
    std::cin >> N;
    for(int i = 0; i < N; i++){
        int n;
        std::cin >> n;
        std::vector<std::pair<float, float>> freckles;
        for(int j = 0; j < n; j++){
            float x, y;
            std::cin >> x >> y;
            freckles.push_back(std::make_pair(x, y));
        }

    }

    return 0;
}