// Created: 2022-08-10
// Link: https://open.kattis.com/problems/freckles

//Problem type: Minimum spanning tree
//Solution: 

// Kattis allows all standard libraries included in C++
// #include <algorithm>
#include <iostream>
// #include <string>
#include <vector>
#include <cmath>

float distance(std::pair<float,float> p1, std::pair<float,float> p2){
    return sqrt(pow(p1.first-p2.first,2)+pow(p1.second-p2.second,2));
}

float shortest_path_alg(std::vector<std::pair<float,float>> points){
    // Using the FLoyd-Warshall algorithm to find the shortest path between all points
    // https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
    float shortest_path = 0;
    int n = points.size();
    std::vector<std::vector<float>> dist(n, std::vector<float>(n, 0));
    for(int i = 0; i < n; i++){
        for(int j = 0; j < n; j++){
            dist[i][j] = distance(points[i], points[j]);
        }
    }


    return shortest_path;
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
        std::cout << shrt_path << std::endl;
    }

    return 0;
}