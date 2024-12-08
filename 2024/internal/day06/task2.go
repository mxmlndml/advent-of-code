package day06

import (
	"fmt"
	"slices"

	"github.com/mxmlndml/advent-of-code/2024/internal/inputs"
)

type point struct {
	row int
	col int
}

type steps struct {
	up    []point
	right []point
	down  []point
	left  []point
}

func (s *steps) add(row int, col int, dir orientationDirection) {
	switch dir {
	case orientationUp:
		(*s).up = append((*s).up, point{row, col})
	case orientationRight:
		(*s).right = append((*s).right, point{row, col})
	case orientationDown:
		(*s).down = append((*s).down, point{row, col})
	case orientationLeft:
		(*s).left = append((*s).left, point{row, col})
	}
}

func (s steps) cyclePossible(row int, col int, dir orientationDirection, lines []string) bool {
	switch dir {
	case orientationUp:
		dir = orientationRight
	case orientationRight:
		dir = orientationDown
	case orientationDown:
		dir = orientationLeft
	case orientationLeft:
		dir = orientationUp
	}

	if !isMovePossible(row, col, dir, lines) {
		return false
	}
	row, col, dir = move(row, col, dir, lines)

	stepTrace := steps{}
	for {
		pos := point{row, col}
		switch dir {
		case orientationUp:
			if slices.Contains(s.up, pos) || slices.Contains(stepTrace.up, pos) {
				return true
			}
		case orientationRight:
			if slices.Contains(s.right, pos) || slices.Contains(stepTrace.right, pos) {
				return true
			}
		case orientationDown:
			if slices.Contains(s.down, pos) || slices.Contains(stepTrace.down, pos) {
				return true
			}
		case orientationLeft:
			if slices.Contains(s.left, pos) || slices.Contains(stepTrace.left, pos) {
				return true
			}
		}

		stepTrace.add(row, col, dir)

		if !isMovePossible(row, col, dir, lines) {
			fmt.Println(false)
			return false
		}
		row, col, dir = move(row, col, dir, lines)
	}
}

func Task2(demo bool) {
	lines := inputs.LoadInputAsLines(6, 1, demo)
	row, col, dir := getGuard(lines)

	stepTrace := steps{}
	obstacles := 0
	for {
		lines[row] = lines[row][:col] + "X" + lines[row][col+1:]
		stepTrace.add(row, col, dir)

		if !isMovePossible(row, col, dir, lines) {
			break
		}

		if stepTrace.cyclePossible(row, col, dir, lines) {
			obstacles++
		}

		row, col, dir = move(row, col, dir, lines)
	}

	fmt.Println(obstacles)
}
