#ifndef __DAY15_H
#define __DAY15_H

// Struct declarations

typedef struct Num
{
    int val;
    int pos;
} Num;

typedef struct HashSet
{
    Num *arr;
    int size;
    int capacity;
} HashSet;

// Returns an address to a newly allocated hashset
HashSet *init_hashset(int capacity);

// Insert a Num struct into the hash
void *hash_insert(HashSet *hash, Num *num);

// Returns a deterministic number given array capcity and value
unsigned hash_func(int val, int capacity);

// Returns an address to a newly allocated Num
Num *init_num(int val, int pos);

// Returns the smallest prime number larger than n
int next_largest_prime(int n);

#endif