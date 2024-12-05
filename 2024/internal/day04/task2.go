package day04

import (
	"fmt"
	"strings"

	"github.com/mxmlndml/advent-of-code/2024/internal/inputs"
)

func Task2(demo bool) {
	lines := inputs.LoadInputAsLines(4, 1, demo)

	sum := 0

	for row, line := range lines {
		strings.Index(line, "A")
		column := 0

		for {
			j := strings.Index(line, "A")
			if j == -1 {
				break
			}
			column += j

			if topLeftBottomRight(row, column, lines) && topRightBottomLeft(row, column, lines) {
				sum++
			}

			line = line[j+1:]
			column += 1
		}
	}

	fmt.Println(sum)
}

func topLeftBottomRight(row int, column int, lines []string) bool {
	if row == 0 || row == len(lines)-1 || column == 0 || column == len(lines)-1 {
		return false
	}
	topLeft := string(lines[row-1][column-1])
	bottomRight := string(lines[row+1][column+1])
	return (topLeft == "S" && bottomRight == "M") || (topLeft == "M" && bottomRight == "S")
}

func topRightBottomLeft(row int, column int, lines []string) bool {
	if row == 0 || row == len(lines)-1 || column == 0 || column == len(lines)-1 {
		return false
	}
	topRight := string(lines[row-1][column+1])
	bottomLeft := string(lines[row+1][column-1])
	return (topRight == "S" && bottomLeft == "M") || (topRight == "M" && bottomLeft == "S")
}
