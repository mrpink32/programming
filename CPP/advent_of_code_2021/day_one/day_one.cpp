#include<iostream>
#include<fstream>
#include<string>


int main()
{
    std::ifstream file("DepthData.txt");
    int data[2000];
    std::string myText;
    while (std::getline(file, myText))
    {
        //data->append(std::stoi(myText));
        
        std::cout << myText << std::endl;
    }
    file.close();
    int previousNumber = data[0];
    int increaseCount = 0;
    for (size_t i = 0; i < std::size(data); i++)
    {
        
    }
    





}