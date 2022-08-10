// Created: 2022-08-09
// Link: https://open.kattis.com/problems/knapsack

// Kattis allows all standard libraries included in C++
#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

std::vector<std::vector<int>> knapsack_algorithm(int C, int n, std::vector<std::pair<int,int>> testCase){
    // testCase.first = value
    // testCase.second = weight
    std::vector<std::vector<int>> dp(n+1, std::vector<int>(C+1, 0));
    for(int i = 1; i <= n; i++){
        for(int j = 1; j <= C; j++){
            if(testCase[i-1].second > j){
                dp[i][j] = dp[i-1][j];
            }
            else{
                dp[i][j] = std::max(dp[i-1][j], dp[i-1][j-testCase[i-1].second] + testCase[i-1].first);
            }
        }
    }

    return dp;
}

void generate_output(std::vector<std::vector<int>> dp, int n, int C, std::vector<std::pair<int,int>> testCase){
    int i = n;
    int j = C;
    while(i > 0 && j > 0){
        if(dp[i][j] == dp[i-1][j]){
            i--;
        }
        else{
            std::cout << i << " ";
            j -= testCase[i].second;
            i--;
        }
    }
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
        std::vector<std::vector<int>> dp = knapsack_algorithm(capacity, numberOfItems, testCase);
        // generate_output(dp, numberOfItems, capacity, testCase);
    }

    return 0;
}