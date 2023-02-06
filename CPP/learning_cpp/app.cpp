#include <iostream>
#include <stdio.h>

class vector3
{
public:
    float x, y, z;
    vector3(float x, float y, float z)
    {
        this->x = x;
        this->y = y;
        this->z = z;
    }
};

struct vector2
{
    float x;
    float y;
};

void printf(vector3 *pos)
{
    cout << pos->x << " " << pos->y << " " << pos->z << endl;
}

void printf(vector2 *pos)
{
    cout << pos->x << " " << pos->y << endl;
}

void sizes() {
    std::cout << "\nsize of bool: " << sizeof(bool) << std::endl;
    std::cout << "size of char: " << sizeof(char) << std::endl;
    std::cout << "size of int: " << sizeof(int) << std::endl;
    std::cout << "size of float: " << sizeof(float) << std::endl;
    std::cout << "size of long: " << sizeof(long) << std::endl;
    std::cout << "size of long long: " << sizeof(long long) << std::endl;
    std::cout << "size of double: " << sizeof(double) << std::endl;
    std::cout << "size of long double: " << sizeof(long double) << std::endl;
}

int main()
{
    vector3 pos(4.6f, 6.0f, 7.56f);
    vector2 pos2 = {4.6f, 6.0f};
    printf(&pos);
    printf(&pos2);
    FILE* data_write = fopen("data", "w");
    printf("%d\n", fileno(data_write));
    fprintf(data_write, "localhost 9000\n");
    fclose(data_write);
    FILE *data_read = fopen("data", "r");
    const char* ip[9];
    long double port;
    fscanf(data_read, "%s %Lf", &ip, &port);
    printf("%s %Lf", &ip, &port);
    fclose(data_read);
    sizes();
    if (-1) {
        std::cout << "true" << std::endl;
    }
}
