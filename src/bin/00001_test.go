package project_euler

import (
	"fmt"
	"testing"
	"lib"
)

func S1(n int) (sum int) {
	sum = 0
	for i := 1; i < n; i++ {
		if i%3 == 0 || i%5 == 0 {
			sum += i
		}
	}
	return
}

func Test1(t *testing.T) {
	time := lib.StartTimer()
	sum := S1(1000)
	lib.StopTimer(time)
	fmt.Printf("\nsum = %d\n", sum)
}

func Benchmark1(b *testing.B) {
	for i := 0; i < b.N; i++ {
		S1(1000)
	} 
}