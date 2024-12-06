package day06

import (
	"fmt"
	"log"
	"strings"

	"github.com/mxmlndml/advent-of-code/2024/internal/inputs"
)

type orientationDirection int

const (
	orientationUp orientationDirection = iota
	orientationRight
	orientationDown
	orientationLeft
)

var orientation = map[orientationDirection]string{
	orientationUp:    "up",
	orientationRight: "right",
	orientationDown:  "down",
	orientationLeft:  "left",
}

func (o orientationDirection) String() string {
	return orientation[o]
}

func Task1(demo bool) {
	lines := inputs.LoadInputAsLines(6, 1, demo)
	row, col, dir := getGuard(lines)
	maxCol := len(lines[0]) - 1
	maxRow := len(lines) - 1

	for {
		lines[row] = lines[row][:col] + "X" + lines[row][col+1:]

		if (dir == orientationUp && row == 0) ||
			(dir == orientationRight && col == maxCol) ||
			(dir == orientationDown && row == maxRow) ||
			(dir == orientationLeft && col == 0) {
			break
		}

		switch dir {
		case orientationUp:
			if lines[row-1][col] == '#' {
				dir = orientationRight
				continue
			}
			row--
		case orientationRight:
			if lines[row][col+1] == '#' {
				dir = orientationDown
				continue
			}
			col++
		case orientationDown:
			if lines[row+1][col] == '#' {
				dir = orientationLeft
				continue
			}
			row++
		case orientationLeft:
			if lines[row][col-1] == '#' {
				dir = orientationUp
				continue
			}
			col--
		}
	}

	steps := 0
	for _, line := range lines {
		steps += strings.Count(line, "X")
	}
	fmt.Println(steps)
}

func getGuard(lines []string) (int, int, orientationDirection) {
	for row, line := range lines {
		column := strings.IndexFunc(line, func(r rune) bool {
			return r == '^' || r == '>' || r == 'v' || r == '<'
		})
		if column != -1 {
			var dir orientationDirection
			switch line[column] {
			case '^':
				dir = orientationUp
			case '>':
				dir = orientationRight
			case 'v':
				dir = orientationDown
			case '<':
				dir = orientationLeft
			}
			return row, column, dir
		}
	}
	log.Fatal("no guard found")
	return -1, -1, -1
}
