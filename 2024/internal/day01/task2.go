package day01

import (
	"fmt"

	"github.com/mxmlndml/advent-of-code/2024/internal/inputs"
)

func Task2(demo bool) {
	lines := inputs.LoadInputAsLines(1, 1, demo)
	leftIDs, rightIDs := getSortedLists(lines)
	fmt.Println(getSimilarityScore(leftIDs, rightIDs))
}

func getSimilarityScore(leftIDs, rightIDs []int) int {
	sum := 0

	for i := range leftIDs {
		leftID := leftIDs[i]
		count := 0
		for j := range rightIDs {
			if rightIDs[j] == leftID {
				count++
			}
		}
		sum += leftID * count
	}

	return sum
}
