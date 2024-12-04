package day02

import (
	"fmt"

	"github.com/mxmlndml/advent-of-code/2024/internal/inputs"
)

func Task2(demo bool) {
	reports := inputs.LoadInputAsLines(2, 1, demo)

	safeReports := 0
	for _, report := range reports {
		levels := getLevels(report)
		if isSafeWithDampener(levels) {
			safeReports++
		}
	}

	fmt.Println(safeReports)
}

func isSafeWithDampener(levels []int) bool {
	safe, errorIndizes := isSafe(levels)
	if safe {
		return true
	}

	for _, i := range errorIndizes {
		if i < 0 {
			continue
		}

		shortenedLevels := copySliceWithoutIndex(levels, i)
		safe, _ = isSafe(shortenedLevels)
		if safe {
			return true
		}

		shortenedLevels = copySliceWithoutIndex(levels, i-1)
		safe, _ = isSafe(shortenedLevels)
		if safe {
			return true
		}
	}

	return false
}

func copySliceWithoutIndex(slice []int, index int) []int {
	result := make([]int, 0, len(slice)-1)
	for i, v := range slice {
		if index == i {
			continue
		}
		result = append(result, v)
	}

	return result
}
