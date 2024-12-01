package days

import (
	"fmt"
	"log"
	"math"
	"slices"
	"strconv"
	"strings"

	"github.com/mxmlndml/advent-of-code/2024/internal"
)

func Day01(task int, demo bool) {
	switch task {
	case 1:
		task1(demo)
		break
	case 2:
		task2(demo)
		break
	}
}

func task1(demo bool) {
	lines := internal.LoadInputAsLines(1, 1, demo)
	leftIDs, rightIDs := getSortedLists(lines)
	fmt.Println(getTotalDistance(leftIDs, rightIDs))
}

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

func getTotalDistance(leftIDs, rightIDs []int) int {
	sum := 0

	for i := range leftIDs {
		leftID := leftIDs[i]
		rightID := rightIDs[i]

		sum += int(math.Abs(float64(leftID - rightID)))
	}

	return sum
}

func task2(demo bool) {
	lines := internal.LoadInputAsLines(1, 1, demo)
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
