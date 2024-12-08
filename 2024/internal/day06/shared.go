package day06

import (
	"log"
	"strings"
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

func isMovePossible(row int, col int, dir orientationDirection, lines []string) bool {
	maxCol := len(lines[0]) - 1
	maxRow := len(lines) - 1
	return !((dir == orientationUp && row == 0) ||
		(dir == orientationRight && col == maxCol) ||
		(dir == orientationDown && row == maxRow) ||
		(dir == orientationLeft && col == 0))
}

func move(row int, col int, dir orientationDirection, lines []string) (int, int, orientationDirection) {
	switch dir {
	case orientationUp:
		if lines[row-1][col] == '#' {
			dir = orientationRight
			break
		}
		row--
	case orientationRight:
		if lines[row][col+1] == '#' {
			dir = orientationDown
			break
		}
		col++
	case orientationDown:
		if lines[row+1][col] == '#' {
			dir = orientationLeft
			break
		}
		row++
	case orientationLeft:
		if lines[row][col-1] == '#' {
			dir = orientationUp
			break
		}
		col--
	}

	return row, col, dir
}
