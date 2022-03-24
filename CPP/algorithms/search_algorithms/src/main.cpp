#include <iostream>
#include <random>
#include <chrono>
#define RAND_MAX INT_MAX;
using namespace std::chrono;

void getNumericInput(int& input, const char* text)
{
	// https://stackoverflow.com/questions/10828937/how-to-make-cin-take-only-numbers
	std::cout << text;
	while (!(std::cin >> input))
	{
		std::cin.clear();
		std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
		std::cout << text;
	}
}

std::vector<int> createVectorData()
{
	int minValue, maxValue, size;
	getNumericInput(size, "Enter desired array size: ");
	getNumericInput(minValue, "Enter desired minnimum value: ");
	getNumericInput(maxValue, "Enter desired maximum value: ");
	std::vector<int> input(size);
	for (int i = 0; i < size; i++)
	{
		//std::srand(std::time(NULL));
		input[i] = rand() % maxValue + minValue;
	}
	return input;
}

void swapVariables(int& x, int&y)
{
	int temp = x;
	x = y;
	y = temp;
}

#pragma region sortingAlgorithms
std::vector<int> bubbleSort(std::vector<int> input, int& n)
{
	int i, j;
	for (i = 0; i < n - 1; i++)
	// Last i elements are already in place 
	for (j = 0; j < n - i - 1; j++)
		if (input[j] > input[j + 1])
			swapVariables(input[j], input[j + 1]);
	return input;
}
#pragma endregion
#pragma region sortedSearchAlgorithms
std::vector<int> linearSearch(std::vector<int> input, int& n,  int& target) {
	for (int i = 0; i < n; i++) {
		if ((input[i]) == target) {
			std::vector<int> result = { i, i };
			return result;
		}
	}
	std::vector<int> result = { NULL, NULL };
	return result;
}
std::vector<int> binarySearch(std::vector<int> input, int& n, int& target) {
	int aC = 0;
	int i = 0;
	int j = n - 1;
	while (i != j)
	{
		int m = floor((i + j) / 2);
		if (input[m] < target)
		{
			//std::cout << "too small" << std::endl;
			i = m + 1;
			aC++;
		}
		else if (input[m] > target)
		{
			//std::cout << "too big" << std::endl;
			j = m - 1;
			aC++;
		}
		else
		{
			aC++;
			std::vector<int> result = { m, aC };
			return result;
		}
	}
	std::vector<int> result = { i, aC };
	return result;
}
#pragma endregion

int main() {
	std::vector<int> input = createVectorData();
	int size = std::size(input);
	for (int i : input) {
		std::cout << i << " ";
	}
	std::cout << std::endl;
	input = bubbleSort(input, size);
	for (int i : input) {
		std::cout << i << " ";
	}
	std::cout << std::endl;
	int target;
	getNumericInput(target, "Input target value: ");
	auto start = high_resolution_clock::now();
	std::vector<int> index = linearSearch(input, size, target);
	auto stop = high_resolution_clock::now();
	auto duration = duration_cast<microseconds>(stop - start);
	std::cout << "Target found at index: " << index[0] << " with execution time of: " << duration.count() << " microseconds and operation count of: " << index[1] << std::endl;
	start = high_resolution_clock::now();
	index = binarySearch(input, size, target);
	stop = high_resolution_clock::now();
	duration = duration_cast<microseconds>(stop - start);
	std::cout << "Target found at index: " << index[0] << " with execution time of: " << duration.count() << " microseconds and operation count of: " << index[1] << std::endl;
}




