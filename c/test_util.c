#include "test_util.h"

#define true 1
#define false 0

#include "ctype.h"
#include "string.h"

// Count the number of numbers in a string.
int count_nums(char *str)
{
    int num = 0;
    int in_number = false;

    for (size_t i = 0; i < strlen(str); i++)
    {
        if (isdigit(str[i]))
        {
            if (!in_number)
            {
                num++;
                in_number = true;
            }
        }
        else
        {
            in_number = false;
        }
    }

    return num;
}

// Returns dynamic array of numbers from a string.
int* get_nums(char *str, int *size)
{
    *size = count_nums(str);
    int *num = (int*)malloc(*size * sizeof(int));
    CHECK(num);
    int in_number = false;

    int cur = 0;
    for (size_t i = 0; i < strlen(str); i++)
    {
        if (isdigit(str[i]) || str[i] == '-')
        {
            if (!in_number)
            {
                num[cur++] = atoi(&str[i]);
                in_number = true;
            }
        }
        else
        {
            in_number = false;
        }
    }
    CHECK(cur==*size);

    return num;
}

void test_helper_functions()
{
    // Check number parsing.
    char *str = "";
    int test = count_nums(str);
    CHECK(0 == test, "string:'%s', numbers: %d\n", str, test);

    str = " ";
    test = count_nums(str);
    CHECK(0 == test, "string:'%s', numbers: %d\n", str, test);

    str = "1";
    test = count_nums(str);
    CHECK(1 == test, "string:'%s', numbers: %d\n", str, test);

    str = " 1";
    test = count_nums(str);
    CHECK(1 == test, "string:'%s', numbers: %d\n", str, test);

    str = " 1 ";
    test = count_nums(str);
    CHECK(1 == test, "string:'%s', numbers: %d\n", str, test);

    str = "44 56 23";
    test = count_nums(str);
    CHECK(3 == test, "string:'%s', numbers: %d\n", str, test);

    str = " -44        56 23 ";
    test = count_nums(str);
    CHECK(3 == test, "string:'%s', numbers: %d\n", str, test);

    // Test returning a dynamic array of numbers from a string.
    int size;
    int *num = get_nums(str, &size);
    CHECK(3 == size, "string:'%s', numbers: %d\n", str, size);
    CHECK(-44 == num[0], "expected: %d actual %d\n", -44, num[0]);
    CHECK(56 == num[1], "expected: %d actual %d\n", 56, num[1]);
    CHECK(23 == num[2], "expected: %d actual %d\n", 23, num[2]);
    free(num);
}
