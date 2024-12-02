#include <fstream>
#include <iostream>
#include <sstream>
#include <vector>

bool is_safe(std::vector<int> levels) {
  bool increasing = true, decreasing = true;

  for (int i = 1; i < levels.size(); i++) {
    int diff = levels[i] - levels[i - 1];
    if (std::abs(diff) < 1 || std::abs(diff) > 3)
      return false;

    if (diff < 0)
      increasing = false;
    if (diff > 0)
      decreasing = false;
  }

  return increasing || decreasing;
}

bool problem_dampener(std::vector<int> levels) {
  for (int i = 0; i < levels.size(); i++) {
    std::vector<int> modified = levels;
    modified.erase(modified.begin() + i);

    if (is_safe(modified))
      return true;
  }
  return false;
}

int main() {
  std::ifstream input_file("input");

  if (!input_file) {
    std::cout << "Couldn't open file" << std::endl;
    return 1;
  }

  std::vector<std::vector<int>> reports;
  std::string line;

  while (getline(input_file, line)) {
    std::stringstream ss(line);
    std::vector<int> levels;
    int level;

    while (ss >> level) {
      levels.push_back(level);
    }

    reports.push_back(levels);
  }

  input_file.close();

  int part_1 = 0;

  for (std::vector<int> report : reports) {
    bool safe = is_safe(report);
    if (safe)
      part_1++;
  }

  std::cout << "Part 1: " << part_1 << std::endl;

  int part_2 = 0;

  for (std::vector<int> report : reports) {
    bool safe = is_safe(report) || problem_dampener(report);
    if (safe)
      part_2++;
  }

  std::cout << "Part 2: " << part_2 << std::endl;

  return 0;
}
