
#include <iostream>

int main()
{
    char conversion;
    std::cout << "Enter the number assigned to the desired currency conversion: \n1 = DKK to USD\n2 = meh\n";
    std::cin >> conversion;
    switch (conversion)
    {
    case 1:
        std::cout << "dkk to usd";
        break;
    case 2:
        std::cout << "meh";
        break;
    default:
        break;
    }
    return 0;
}
