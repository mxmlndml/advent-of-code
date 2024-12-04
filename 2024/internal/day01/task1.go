package day01

import (
	"fmt"
	"math"

	"github.com/mxmlndml/advent-of-code/2024/internal/inputs"
)

func Task1(demo bool) {
	lines := inputs.LoadInputAsLines(1, 1, demo)
	leftIDs, rightIDs := getSortedLists(lines)
	fmt.Println(getTotalDistance(leftIDs, rightIDs))
}

func getTotalDistance(leftIDs, rightIDs []int) int {
	sum := 0

	for i := range leftIDs {
		leftID := leftIDs[i]
		rightID := rightIDs[i]

		sum += int(math.Abs(float64(leftID - rightID)))
	}

	return sum
}
