package main

import (
	"fmt"
	"lib"
)

func main() {
	t := lib.StartTimer()

	sum := 0
	for i := 1; i < 1000; i++ {
		if i%3 == 0 || i%5 == 0 {
			sum += i
		}
	}
	fmt.Printf("sum = %d\n", sum)

	lib.StopTimer(t)
}
