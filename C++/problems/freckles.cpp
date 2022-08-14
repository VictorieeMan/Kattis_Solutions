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
#include <tuple>

double round_n(double x, int n = 2) {
    return roundf(x * pow(10, n)) / pow(10, n);
}

double distance(std::pair<double,double> p1, std::pair<double,double> p2){
    return sqrt(pow(p1.first-p2.first,2)+pow(p1.second-p2.second,2));
}

double node_distance(std::tuple<int, double, double> n1, std::tuple<int, double, double> n2){
    std::pair<double,double> p1 = std::make_pair(std::get<1>(n1), std::get<2>(n1));
    std::pair<double,double> p2 = std::make_pair(std::get<1>(n2), std::get<2>(n2));
    return distance(p1, p2);
}

double shortest_path_alg(std::vector<std::pair<double,double>> points){
    // Greedy algorithm to find the shortest path
    std::vector<std::pair<double,double>> visited;
    visited.push_back(points.back());
    points.pop_back();

    double min_dist = 0;
    int n = points.size();

    for(int i=0; i<n;i++){
        std::vector<double> distances(points.size());
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

// Prim algorithm to find the shortest path
double minimal_spanning_tree_prim(std::vector<std::tuple<int, double, double>> points){
    double mst_len = 0;
    return mst_len;
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
        std::vector<std::tuple<int, double, double>> freckles_index;
        for(int j = 0; j < n; j++){
            double x, y;
            std::cin >> x >> y;
            freckles_index.push_back(std::make_tuple(j, x, y));
        }
        double shrt_path = minimal_spanning_tree_prim(freckles_index);
        std::cout << round_n(shrt_path) << std::endl;
    }

    return 0;
}