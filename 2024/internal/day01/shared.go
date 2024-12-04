package day01

import (
	"log"
	"slices"
	"strconv"
	"strings"
)

func getSortedLists(lines []string) ([]int, []int) {
	leftIDs := make([]int, 0, len(lines))
	rightIDs := make([]int, 0, len(lines))

	for i := range lines {
		line := lines[i]
		ids := strings.Split(line, "   ")
		leftID, err := strconv.Atoi(ids[0])
		if err != nil {
			log.Fatal(err)
		}
		rightID, err := strconv.Atoi(ids[1])
		if err != nil {
			log.Fatal(err)
		}
		leftIDs = append(leftIDs, leftID)
		rightIDs = append(rightIDs, rightID)
	}

	slices.SortFunc(leftIDs, func(a, b int) int { return a - b })
	slices.SortFunc(rightIDs, func(a, b int) int { return a - b })

	return leftIDs, rightIDs
}
