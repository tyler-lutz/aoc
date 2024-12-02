#include <algorithm>
#include <fstream>
#include <iostream>
#include <unordered_map>
#include <vector>

int calculate_total_distance(std::vector<int> left, std::vector<int> right) {
  int total_distance = 0;

  for (int i = 0; i < left.size(); i++) {
    total_distance += std::abs(left[i] - right[i]);
  }

  return total_distance;
}

int similarity_score(std::vector<int> left, std::vector<int> right) {
  std::unordered_map<int, int> right_frequency;
  for (int num : right) {
    right_frequency[num]++;
  }

  int score = 0;
  for (int num : left) {
    score += num * right_frequency[num];
  }

  return score;
}

int main() {
  std::ifstream input_file("input");

  if (!input_file) {
    std::cout << "Couldn't open file" << std::endl;
    return 1;
  }

  std::vector<int> left_list, right_list;
  int left, right;

  while (input_file >> left >> right) {
    left_list.push_back(left);
    right_list.push_back(right);
  }

  input_file.close();

  std::sort(left_list.begin(), left_list.end());
  std::sort(right_list.begin(), right_list.end());

  int part1 = calculate_total_distance(left_list, right_list);
  int part2 = similarity_score(left_list, right_list);

  std::cout << "Part 1: " << part1 << std::endl;
  std::cout << "Part 2: " << part2 << std::endl;

  return 0;
}
