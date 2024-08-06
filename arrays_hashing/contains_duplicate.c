#include <stdio.h>
#include <stdbool.h>

bool containsDuplicate(int* nums, int numsSize) {
    int i;
    int j;

    for (i = 0; i < numsSize; i++) { 
        for (j = 0; j < numsSize; j++) {
            if (i == j) {
                continue;
            }
            if (
                *(nums + i) == *(nums + j)
            ) {
                return true;
            }
        }
    }

    return false;
}

int main(int argc, char const *argv[])
{
    return 0;
}
