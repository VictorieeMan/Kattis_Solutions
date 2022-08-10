// Created: 2022-08-10
// Link: https://open.kattis.com/problems/freckles

//Problem type: Minimum spanning tree
//Solution: 

// Kattis allows all standard libraries included in C++
#include <algorithm>
#include <iostream>
#include <string>
#include <vector>
#include <cmath>

float round_n(float x, int n = 2) {
    return roundf(x * pow(10, n)) / pow(10, n);
}

float distance(std::pair<float,float> p1, std::pair<float,float> p2){
    return sqrt(pow(p1.first-p2.first,2)+pow(p1.second-p2.second,2));
}

float shortest_path_alg(std::vector<std::pair<float,float>> points){
    // Greedy algorithm to find the shortest path
    std::vector<std::pair<float,float>> visited;
    float min_dist = 0;
    int n = points.size()-1;
    visited.push_back(points.back());
    points.pop_back();
    for(int i=0; i<n;i++){
        std::vector<float> distances(points.size());
        for(int j=0; j<points.size();j++){
            distances[j] = distance(visited.back(),points[j]);
        }
        int min_index = std::min_element(distances.begin(),distances.end()) - distances.begin();
        min_dist += distances[min_index];
        visited.push_back(points[min_index]);
        points.erase(points.begin()+min_index);
    }

    return min_dist;
}

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
        float shrt_path = shortest_path_alg(freckles);
        std::cout << round_n(shrt_path) << std::endl;
    }

    return 0;
}