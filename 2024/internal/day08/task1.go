package day08

import (
	"fmt"

	"github.com/mxmlndml/advent-of-code/2024/internal/inputs"
)

type point struct {
	row int
	col int
}

func (p point) diff(q point) point {
	return point{
		p.row - q.row,
		p.col - q.col,
	}
}

type set[V comparable] map[V]struct{}

func (s *set[V]) add(v V) {
	(*s)[v] = struct{}{}
}
func (s *set[V]) del(v V) {
	delete((*s), v)
}

func Task1(demo bool) {
	lines := inputs.LoadInputAsLines(8, 1, demo)
	antinodes := make(set[point])

	for row, line := range lines {
		for col, antenna := range line {
			if antenna == '.' {
				continue
			}
			addAntinodes(antenna, point{row, col}, lines, &antinodes)
		}
	}

	maxRow := len(lines) - 1
	maxCol := len(lines[0]) - 1
	for antinode := range antinodes {
		if antinode.col < 0 || antinode.col > maxCol || antinode.row < 0 || antinode.row > maxRow {
			antinodes.del(antinode)
		}
	}

	fmt.Println(len(antinodes))
}

func addAntinodes(antenna rune, pos point, lines []string, antinodes *set[point]) {
	for col := pos.col + 1; col < len(lines[pos.row]); col++ {
		antenna2 := rune(lines[pos.row][col])
		if antenna == antenna2 {
			diff := col - pos.col

			antinodes.add(point{pos.row, pos.col - diff})
			antinodes.add(point{pos.row, col + diff})
		}
	}

	for row := pos.row + 1; row < len(lines); row++ {
		for col := 0; col < len(lines[row]); col++ {
			antenna2 := rune(lines[row][col])
			if antenna == antenna2 {
				diff := point{row - pos.row, col - pos.col}

				antinodes.add(point{pos.row - diff.row, pos.col - diff.col})
				antinodes.add(point{row + diff.row, col + diff.col})
			}
		}
	}
}
