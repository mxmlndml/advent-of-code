package internal

import (
	"fmt"

	"github.com/mxmlndml/advent-of-code/2024/internal/day01"
	"github.com/mxmlndml/advent-of-code/2024/internal/day02"
)

func Run(day int, task int, demo bool) {
	switch day {
	case 1:
		switch task {
		case 1:
			day01.Task1(demo)
			return
		case 2:
			day01.Task2(demo)
			return
		}
	case 2:
		switch task {
		case 1:
			day02.Task1(demo)
			return
		case 2:
			day02.Task2(demo)
			return
		}
	default:
		fmt.Printf("Day %d has not been implemented yet\n", day)
	}
}
