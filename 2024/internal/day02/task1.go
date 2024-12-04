package day02

import (
	"fmt"

	"github.com/mxmlndml/advent-of-code/2024/internal/inputs"
)

func Task1(demo bool) {
	reports := inputs.LoadInputAsLines(2, 1, demo)

	safeReports := 0
	for _, report := range reports {
		levels := getLevels(report)
		safe, _ := isSafe(levels)
		if safe {
			safeReports++
		}
	}

	fmt.Println(safeReports)
}
