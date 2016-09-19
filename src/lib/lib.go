package lib

import (
    "time"
    "fmt"
)

func StartTimer() time.Time {
    return time.Now()
}

func StopTimer(t time.Time) {
    fmt.Printf("time elapse %f second", time.Since(t).Seconds())
}