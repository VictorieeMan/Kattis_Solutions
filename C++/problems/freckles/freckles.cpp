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

// Insert new edge in ascending order of distances
std::vector<std::tuple<int, int, double>> insert_new_edge_ascOrder(
    std::vector<std::tuple<int, int, double>>& edges,
    std::tuple<int, int, double> new_edge
    ){
        int n = edges.size();
        if(n==0){
            edges.push_back(new_edge);
            return edges;
        } else{
            for(int i = 0; i<n; i++){
                if(std::get<2>(new_edge) < std::get<2>(edges[i])){
                    edges.insert(edges.begin()+i, new_edge);
                    return edges;
                }
            }
        }
        return edges;
}

// Check if edge is calculated
bool edge_is_calculated(int a, int b, std::vector<std::tuple<int, int, double>> edges){
    for(int i=0; i<edges.size();i++){
        if(std::get<0>(edges[i]) == a && std::get<1>(edges[i]) == b){
            return true;
        }
        if(std::get<0>(edges[i]) == b && std::get<1>(edges[i]) == a){
            return true;
        }
    }
    return false;
}

// Calculating the spokes of a node, to all free_nodes, and updates the list of edges.
void calculate_spokes(
    std::vector<std::tuple<int, int, double>>& edges, 
    std::vector<std::tuple<int, double, double>> free_nodes, 
    std::tuple<int, double, double> node
    ){
    int n = free_nodes.size();
    for(int i=0; i<n;i++){
        std::tuple<int, int, double> new_edge;
        if(!edge_is_calculated(std::get<0>(node), std::get<0>(free_nodes[i]), edges)){
            new_edge = std::make_tuple(std::get<0>(node), std::get<0>(free_nodes[i]), node_distance(node, free_nodes[i]));
            edges = insert_new_edge_ascOrder(edges, new_edge);
        }
    }
}

// Get the edge, from index pair
std::tuple<int, int, double> get_edge(int a, int b, std::vector<std::tuple<int, int, double>> edges){
    for(int i=0; i<edges.size();i++){
        if(std::get<0>(edges[i]) == a && std::get<1>(edges[i]) == b){
            return edges[i];
        }
        if(std::get<0>(edges[i]) == b && std::get<1>(edges[i]) == a){
            return edges[i];
        }
    }
    return std::make_tuple(0,0,0);
}

// Get the edge length, from index pair
double get_edge_length(int a, int b, std::vector<std::tuple<int, int, double>> edges){
    return std::get<2>(get_edge(a, b, edges));
}

// Check if node is part of a measured edge
bool node_is_measured(int a, std::vector<std::tuple<int, int, double>> edges){
    for(int i=0; i<edges.size();i++){
        if(std::get<0>(edges[i]) == a || std::get<1>(edges[i]) == a){
            return true;
        }
    }
    return false;
}

// Check if edge contains free node
bool edge_contains_free_node(
    std::tuple<int, int, double> edge, 
    std::vector<std::tuple<int, double, double>> free_nodes
    ){
        for(int i=0; i<free_nodes.size();i++){
            if(std::get<0>(edge) == std::get<0>(free_nodes[i]) || std::get<1>(edge) == std::get<0>(free_nodes[i])){
                return true;
            }
        }
        return false;
}

// Find shortest edge from visited to a free node
std::tuple<int, int, double> find_shortest_new_branch(
    std::vector<std::tuple<int, int, double>> edges, 
    std::vector<std::tuple<int, double, double>> nodes,
    std::vector<std::tuple<int, double, double>> visited, 
    std::vector<std::tuple<int, double, double>> free_nodes
    ){
        std::tuple<int, int, double> new_branch = std::make_tuple(-1,-1,-1);
        int n = edges.size();
        for(int i=0; i<n;i++){
            if(edge_contains_free_node(edges[i], free_nodes)){
                new_branch = edges[i];
                return new_branch;
            }
        }
        return new_branch;
}

// Find node in vectoor, by index
int find_node_in_vector(std::vector<std::tuple<int, double, double>> nodes, int index){
    for(int i=0; i<nodes.size();i++){
        if(std::get<0>(nodes[i]) == index){
            return i;
        }
    }
    return -1;
}


// Prim algorithm to find the shortest path
double minimal_spanning_tree_prim(std::vector<std::tuple<int, double, double>> nodes){
    std::vector<std::tuple<int, int, double>> edges;
    // (node_i_idx, node_j_idx, distance/weight)
    std::tuple<int, int, double> new_edge;

    std::vector<std::tuple<int, double, double>> free_nodes = nodes;
    std::vector<std::tuple<int, double, double>> visited; // Also knows as the tree
    // (node_idx, node_i_x, node_i_y)

    // Initialize the visited nodes with the first node
    visited.push_back(free_nodes.back());
    free_nodes.pop_back();
    double mst_len = 0;
    int i = 0;

    while(free_nodes.size() > 0){
        std::tuple<int, double, double> node_i = visited.back();

        // Calculte the node_i spokes, and add them to list of edges
        calculate_spokes(edges, free_nodes, node_i);
        // Find the shortest edge from the visited nodes to a free node
        new_edge = find_shortest_new_branch(edges, nodes, visited, free_nodes);

        // If no new edge is found, break the loop
        if(std::get<0>(new_edge) == -1){
            std::cout << "No new edge found" << std::endl;
            break;
        }
        
        // Add to the lenght of the minimal spanning tree
        mst_len += std::get<2>(new_edge);

        // Update for next iteration
        int idx = find_node_in_vector(free_nodes, std::get<0>(new_edge));
        if(idx == -1){
            idx = find_node_in_vector(free_nodes, std::get<1>(new_edge));
            if(idx==-1){
                std::cout << "Node not found" << std::endl;
                break;
            }
        }
        visited.push_back(free_nodes[idx]);
        free_nodes.erase(free_nodes.begin() + idx);
        i++;
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
        std::cout << round_n(shrt_path) << std::endl;
    }

    return 0;
}