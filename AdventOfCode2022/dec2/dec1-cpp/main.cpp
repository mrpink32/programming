#include <iostream>
#include <fstream>
#include <string>
#include <vector>

int GetMax(std::vector<int> array)
{
    int maxValue = 0;
    for (size_t i = 0; i < array.size() - 1; i++)
    {
        // std::cout << totals[i] << std::endl;
        if (array[i] >= maxValue)
        {
            maxValue = array[i];
        }
    }
    return maxValue;
}

int main(int argc, char const *argv[])
{
    std::cout << "-----Part1-----" << std::endl;
    std::fstream data;
    data.open("../data");
    std::string line;
    std::vector<int> totals = std::vector<int>();
    if (data.is_open())
    {
        size_t size;
        int total = 0;
        while (getline(data, line))
        {
            if (line.empty())
            {
                totals.push_back(total);
                total = 0;
            }
            else
                total += std::stoi(line, &size);
        }
        data.close();
    }
    else
    {
        std::cout << "Failed to open file" << std::endl;
        return 1;
    }
    int maxValue = GetMax(totals);
    std::cout << maxValue << std::endl;
    std::cout << "-----Part2-----" << std::endl;
    int result = 0;
    for (size_t i = 0; i < 3; i++)
    {
        maxValue = 0;
        size_t targetIndex = 0;
        for (size_t j = 0; j < totals.size() - 1; j++)
        {
            if (totals[j] >= maxValue)
            {
                maxValue = totals[j];
                targetIndex = j;
            }
        }
        result += maxValue;
        totals.erase(totals.begin() + targetIndex);
    }
    std::cout << result << std::endl;
    return 0;
}
