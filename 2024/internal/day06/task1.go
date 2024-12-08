package day06

import (
	"fmt"
	"strings"

	"github.com/mxmlndml/advent-of-code/2024/internal/inputs"
)

func Task1(demo bool) {
	lines := inputs.LoadInputAsLines(6, 1, demo)
	row, col, dir := getGuard(lines)

	for {
		lines[row] = lines[row][:col] + "X" + lines[row][col+1:]

		if !isMovePossible(row, col, dir, lines) {
			break
		}
		row, col, dir = move(row, col, dir, lines)
	}

	steps := 0
	for _, line := range lines {
		steps += strings.Count(line, "X")
	}
	fmt.Println(steps)
}
