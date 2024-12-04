package day02

import (
	"log"
	"math"
	"strconv"
	"strings"
)

func getLevels(report string) []int {
	levels := make([]int, 0)
	for _, char := range strings.Split(report, " ") {
		level, err := strconv.Atoi(char)
		if err != nil {
			log.Fatal(err)
		}
		levels = append(levels, level)
	}
	return levels
}

func isIncreasing(levels []int) (bool, int) {
	for i, level := range levels {
		if i == 0 {
			continue
		}
		if level < levels[i-1] {
			return false, i
		}
	}
	return true, -1
}

func isDecreasing(levels []int) (bool, int) {
	for i, level := range levels {
		if i == 0 {
			continue
		}
		if level > levels[i-1] {
			return false, i
		}
	}
	return true, -1
}

func hasGradualDifference(levels []int) (bool, int) {
	for i, level := range levels {
		if i == 0 {
			continue
		}
		difference := int(math.Abs(float64(level - levels[i-1])))
		if difference < 1 || difference > 3 {
			return false, i
		}
	}
	return true, -1
}

func isSafe(levels []int) (bool, []int) {
	increasing, i := isIncreasing(levels)
	decreasing, j := isDecreasing(levels)
	gradualDifferent, k := hasGradualDifference(levels)
	errorIndizes := []int{i, j, k}

	return (increasing || decreasing) && gradualDifferent, errorIndizes
}
