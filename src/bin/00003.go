package main

import (
	"fmt"
	"lib"
)

const N = 600851475143

func main() {
	t := lib.StartTimer()

	for x, n := 2, N; ; x++ {
		if x == n {
			fmt.Printf("Largest prime factor = %d\n", x)
			break
		}
		n = f(n, x)
	}

	lib.StopTimer(t)
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
