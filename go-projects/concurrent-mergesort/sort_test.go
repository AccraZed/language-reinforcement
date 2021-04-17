package main

import (
	"math/rand"
	"testing"
	"time"
)

func BenchmarkSort(b *testing.B) {
	for i := 0; i < b.N; i++ {
		rand.Seed(time.Now().Unix())
		arr := createRandomArray(b.N, 10*b.N)
		mergeSort(arr)
	}
}
