package project_euler

import (
	"fmt"
    "testing"
	"lib"
)

func S3(max int) int {
	for x, n := 2, max; ; x++ {
		if x == n {
			return x
		}
		n = f(n, x)
	}
}

func f(n, x int) int {
	switch {
    case n == 1 || x == 1:
        return n
	case n%x == 0:
		return f(n/x, x)
    default :
        return n
	}
}

func Test3(t *testing.T) {
    time := lib.StartTimer()
    x := S3(600851475143)
    lib.StopTimer(time)
    fmt.Printf("\nLargest prime factor = %d\n", x)
}

func Benchmark3(b *testing.B) {
    for i := 0; i < b.N; i++ {
        S3(600851475143)
    } 
}
