
#include <iostream>
#include <stdio.h>
#include "moremath.h"
using namespace std;

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

int main()
{
    vector3 pos(4.6f, 6.0f, 7.56f);
    vector2 pos2 = {4.6f, 6.0f};
    printf(&pos);
    printf(&pos2);
    FILE *data_write = fopen("data", "w");
    printf("%d\n", fileno(data_write));
    fprintf(data_write, "localhost 9000\n");
    fclose(data_write);
    FILE *data_read = fopen("data", "r");
    char ip[9];
    int port;
    fscanf(data_read, "%s %d", &ip, &port);
    printf("%s %d", ip, port);
    fclose(data_read);
    return 0;
}
