package main

import (
	"fmt"
	"math/rand"
	"sync"
	"time"
)

const ARR_SIZE = 10000000
const ARR_SCOPE = 100000000
const CONCUR_THRESHOLD = 10000

func main() {
	rand.Seed(time.Now().Unix())
	arr := createRandomArray(ARR_SIZE, ARR_SCOPE)

	fmt.Println("Unsorted: ")
	printArray(arr)

	// Sort Array
	start := time.Now()
	mergeSort(arr)
	elapsed := time.Since(start)

	fmt.Println("Sorted: ")
	printArray(arr)

	fmt.Println("Time Elapsed: ", elapsed)
}

func mergeSort(arr []int) {
	// Base Case: Sorted array of length 1
	if len(arr) == 1 {
		return
	}

	// MergeSort the front and back half of the array
	// Depending on the size of the array, call concurrent vs synchronous function calls
	if len(arr) > CONCUR_THRESHOLD {
		var wg sync.WaitGroup
		wg.Add(2)

		go func() {
			mergeSort(arr[:len(arr)/2])
			wg.Done()
		}()
		go func() {
			mergeSort(arr[len(arr)/2:])
			wg.Done()
		}()

		wg.Wait()
	} else {
		mergeSort(arr[:len(arr)/2])
		mergeSort(arr[len(arr)/2:])
	}
	// Merge both arrays together
	merge(arr[:len(arr)/2], arr[len(arr)/2:])
}

func merge(arr1 []int, arr2 []int) {
	var i, j, k int
	temp := make([]int, len(arr1)+len(arr2))

	// Main merge section
	for i < len(arr1) && j < len(arr2) {
		if arr2[j] < arr1[i] {
			temp[k] = arr2[j]
			k += 1
			j += 1
		} else {
			temp[k] = arr1[i]
			k += 1
			i += 1
		}
	}

	// Append leftover
	for i = i; i < len(arr1); i++ {
		temp[k] = arr1[i]
		k += 1
	}
	for j = j; j < len(arr2); j++ {
		temp[k] = arr2[j]
		k += 1
	}

	// Copy temp into arr1 and arr2
	copy(arr1, temp[:len(arr1)])
	copy(arr2, temp[len(arr1):])
}

func printArray(arr []int) {
	for i := range arr {
		fmt.Printf("%d ", arr[i])
	}
	fmt.Println()
}

// Populate a <len> length array, of integers from 0 to <scope>
func createRandomArray(len int, scope int) []int {
	arr := make([]int, len)

	for i := 0; i < len; i++ {
		arr[i] = rand.Int() % scope
	}

	return arr
}
