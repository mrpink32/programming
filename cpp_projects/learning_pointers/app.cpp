
#include <iostream>
#include <stdio.h>

class vector {
    public:
        float x, y, z;
        vector(float x, float y, float z) {
            this->x = x;
            this->y = y;
            this->z = z;
        }
};

typedef struct vector2 {
    float x;
    float y;
} vector2;

void output(vector *pos) {
    std::cout << pos->x << " " << pos->y << " " << pos->z << std::endl;
}

void output(vector2* pos) {
    std::cout << pos->x << " " << pos->y << std::endl;
}

int main() {
    vector pos(4.6f, 6.0f, 7.56f);
    vector2 pos2 = { 4.6f, 6.0f };
    output(&pos);
    output(&pos2);
    return 0;
}
