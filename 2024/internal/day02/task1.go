package day02

import (
	"fmt"
	"log"
	"math"
	"strconv"
	"strings"

	"github.com/mxmlndml/advent-of-code/2024/internal/inputs"
)

func Task1(demo bool) {
	reports := inputs.LoadInputAsLines(2, 1, demo)

	safeReports := 0
	for _, report := range reports {
		if isSafe(report) {
			safeReports++
		}
	}

	fmt.Println(safeReports)
}

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

func isIncreasing(levels []int) bool {
	for i, level := range levels {
		if i == 0 {
			continue
		}
		if level < levels[i-1] {
			return false
		}
	}
	return true
}

func isDecreasing(levels []int) bool {
	for i, level := range levels {
		if i == 0 {
			continue
		}
		if level > levels[i-1] {
			return false
		}
	}
	return true
}

func hasGradualDifference(levels []int) bool {
	for i, level := range levels {
		if i == 0 {
			continue
		}
		difference := int(math.Abs(float64(level - levels[i-1])))
		if difference < 1 || difference > 3 {
			return false
		}
	}
	return true
}

func isSafe(report string) bool {
	levels := getLevels(report)
	return (isIncreasing(levels) || isDecreasing(levels)) && hasGradualDifference(levels)
}
