#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include "day15.h"

HashSet *init_hashset(int capacity)
{
    HashSet *ret;

    if ((ret = malloc(sizeof(HashSet))) == NULL)
        return NULL;

    if ((ret->arr = malloc(sizeof(Num) * next_largest_prime(capacity * 2))) == NULL)
    {
        free(ret);
        return NULL;
    }

    ret->capacity = capacity;
    ret->size = 0;

    return ret;
}

void *hash_insert(HashSet *hash, Num *num)
{
    int quad_n = 1;
    int index;

    if (hash == NULL || num == NULL)
        return NULL;

    index = hash_func(num->val, hash->capacity);
}

unsigned hash_func(int val, int capacity)
{
    if (val < 0)
        val = -val;

    return (val * ((val * 0xA2 + capacity / 0x95) * 0x2B + val)) % capacity;
}

Num *init_num(int val, int pos)
{
    Num *ret;

    if ((ret = malloc(sizeof(Num))) == NULL)
        return NULL;

    ret->val = val;
    ret->pos = pos;

    return ret;
}

int next_largest_prime(int n)
{
    int i, limit;

    if (n == 1)
        return 2;

    bool keep_looping;

    while (true)
    {
        keep_looping = false;
        limit = ++n / 2;

        for (i = 2; i < limit; i++)
        {
            if (n % i == 0)
            {
                keep_looping = true;
                break;
            }
        }

        if (!keep_looping)
        {
            return n;
        }
    }
}

int main(void)
{
    FILE *ifp;
    HashSet *numbers = init_hashset(2020);
    char buffer[100];
    int pos = 0;

    if ((ifp = fopen("input.txt", "r")) == NULL)
    {
        printf("Couldn't find file...");
        return 1;
    }

    while (fscanf(ifp, "%s", buffer))
    {
        hash_insert(numbers, init_num(atoi(buffer), pos++));
    }

    fclose(ifp);
    return 0;
}