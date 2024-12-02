#include <algorithm>
#include <fstream>
#include <iostream>
#include <vector>

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

  int total_distance = 0;
  for (int i = 0; i < left_list.size(); i++) {
    total_distance += std::abs(left_list[i] - right_list[i]);
  }

  std::cout << "Total Distance: " << total_distance << std::endl;

  return 0;
}
