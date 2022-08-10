// Created: 2022-08-09
// Link: https://open.kattis.com/problems/knapsack

// Problem type: 0-1 knapsack problem
// Solution type: dynamic programming

// Kattis allows all standard libraries included in C++
#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

std::vector<std::vector<int>> knapsack_algorithm(int C, int n, std::vector<std::pair<int,int>> testCase){
    // testCase.first = value
    // testCase.second = weight
    std::vector<std::vector<int>> dpt(n+1, std::vector<int>(C+1, 0));
    for(int i = 1; i <= n; i++){
        for(int j = 1; j <= C; j++){
            if(testCase[i-1].second > j){
                dpt[i][j] = dpt[i-1][j];
            }
            else{
                dpt[i][j] = std::max(dpt[i-1][j], dpt[i-1][j-testCase[i-1].second] + testCase[i-1].first);
            }
        }
    }

    return dpt;
}

void generate_output(std::vector<std::vector<int>> dpt, int n, int C, std::vector<std::pair<int,int>> testCase){
    std::vector<int> output;
    for(int i = n; i > 0; i--){
        if(dpt[i][C] != dpt[i-1][C]){
            output.push_back(i-1);
            // std::cout << i << std::endl;
            // std::cout << testCase[i-1].first << " " << testCase[i-1].second << std::endl;
            C -= testCase[i-1].second;
        }
    }
    std::cout << output.size() << std::endl;
        for(int i = 0; i < output.size(); i++){
            std::cout << output[i] << " ";
        }
        std::cout << std::endl;
}

int main() {
    //Input information:
    // First line in a test case:
    // a Knapsack capacity
    // b Number of items in testcase
    // Following b number of lines:
    // a item value
    // b item weight
    // b+1 line is either non existent or the start of a new test case

    // input
    int capacity, numberOfItems;
    while(std::cin >> capacity >> numberOfItems){
        std::vector<std::pair<int,int>> testCase;
        for(int i = 0; i < numberOfItems; i++){
            int value, weight;
            std::cin >> value >> weight;
            testCase.push_back(std::make_pair(value,weight));
        }
        std::vector<std::vector<int>> dynamic_programming_table = knapsack_algorithm(capacity, numberOfItems, testCase);
        generate_output(dynamic_programming_table, numberOfItems, capacity, testCase);
    }

    return 0;
}