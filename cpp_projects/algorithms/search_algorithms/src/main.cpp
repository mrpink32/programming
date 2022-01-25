#include <iostream>
#include <random>
#include <chrono>
using namespace std::chrono;

#pragma region sorted_search_algorithms
int linearSearch(double input[], int& size,  double& target) {
	for (int i = 0; i < size; i++)
	{
		if ((input[i]) == target) {
			return i;
		}
	}
	return NULL;
}
int binarySearch(double input[], double& target) {
	return 0;
}
#pragma endregion

int main() {
	double input[] = { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50  };
	int size = std::size(input);
	std::srand(std::time(nullptr));
	int index = rand() % 50;
	std::cout << "Result: " << input[index] << " on index: " << index << " in input" << std::endl;
	double target = input[index];
	auto start = high_resolution_clock::now();
	index = linearSearch(input, size, target);
	auto stop = high_resolution_clock::now();
	auto duration = duration_cast<microseconds>(stop - start);
	std::cout << "Result found on index: " << index << " with execution time of: " << duration.count() << std::endl;
	//index = binarySearch(input, target);
	//std::cout << "result found on index: " << index << std::endl;
}




