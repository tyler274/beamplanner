#pragma once

#include "stdlib.h"
#include "stdio.h"

#define BOLD   "\033[1m"
#define GRAY   "\033[38;5;248m"
#define CYAN   "\033[36m"
#define RED    "\033[31m"
#define GREEN  "\033[32m"
#define YELLOW "\033[33m"
#define RESET  "\033[0m"

#define FAIL(...)                                                                                  \
    {                                                                                              \
        printf(BOLD RED "FAIL: " RESET  __VA_ARGS__);                                              \
        printf("\n");                                                                              \
        exit(1);                                                                                   \
    }

#define CHECK(condition, ...)                                                                      \
    if (!(condition))                                                                              \
    {                                                                                              \
        FAIL(__VA_ARGS__);                                                                         \
    }

int count_nums(char *str);
int* get_nums(char *str, int *size);

void test_helper_functions();
