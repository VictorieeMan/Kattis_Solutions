// Created: 2022-08-15
// Link: https://open.kattis.com/problems/freckles

//Problem type: Miniman Spanning Tree
//Solution: Prims algorithm

// Kattis allows all standard libraries included in C++
#include <algorithm>
#include <iostream>
#include <iomanip>
#include <string>
#include <vector>
#include <cmath>
#include <tuple>
#include <set>

double node_distance(int n1, int n2, std::vector<std::tuple<int, double, double>> nodes){
    double p1x = std::get<1>(nodes[n1]);
    double p1y = std::get<2>(nodes[n1]);
    double p2x = std::get<1>(nodes[n2]);
    double p2y = std::get<2>(nodes[n2]);
    return sqrt(pow(p1x-p2x,2)+pow(p1y-p2y,2));
}

double minimal_spanning_tree_prim(
    std::vector<std::tuple<int, double, double>> nodes
    ){
        double mst_len = 0;
        std::vector<std::pair<std::set<int>, double>> edges;
        std::vector<std::tuple<int, double, double>> unvisited = nodes;
        std::vector<int> visited;
        visited.push_back(std::get<0>(unvisited.back()));
        unvisited.pop_back();

        while(unvisited.size() > 0){
            std::vector<double> distances(unvisited.size());
            for(int i=0; i<unvisited.size();i++){
                distances[i] = node_distance(visited.back(), std::get<0>(unvisited[i]), nodes);
            }
            int min_index = std::min_element(distances.begin(),distances.end()) - distances.begin();
            edges.push_back(std::make_pair(std::set<int>{visited.back(), std::get<0>(unvisited[min_index])}, distances[min_index]));
            mst_len += distances[min_index];
            visited.push_back(std::get<0>(unvisited[min_index]));
            unvisited.erase(unvisited.begin()+min_index);
        }

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
        std::cout << std::fixed << std::setprecision(2) << shrt_path << std::endl;
    }

    return 0;
}