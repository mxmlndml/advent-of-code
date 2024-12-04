package day04

import (
	"fmt"
	"strings"

	"github.com/mxmlndml/advent-of-code/2024/internal/inputs"
)

func Task1(demo bool) {
	wordSearch := getWordSearch(demo)

	sum := 0
	for _, line := range strings.Split(wordSearch, "\n") {
		sum += countWord(line, "XMAS")
		sum += countWord(line, "SAMX")
	}

	fmt.Println(sum)
}

func countWord(line string, word string) int {
	sum := 0

	for {
		i := strings.Index(line, word)
		if i == -1 {
			break
		}
		sum++
		line = line[i+1:]
	}

	return sum
}

func getWordSearch(demo bool) string {
	wordSearch := inputs.LoadInput(4, 1, demo)

	lines := inputs.LoadInputAsLines(4, 1, demo)

	columns := make([]string, len(lines[0]))
	for _, line := range lines {
		for column, char := range strings.Split(line, "") {
			columns[column] += char
		}
	}
	wordSearch += "\n"
	wordSearch += strings.Join(columns, "\n")

	diagonalsUp := make([]string, len(lines)*2-1)
	for i, line := range lines {
		for j, char := range strings.Split(line, "") {
			diagonalsUp[i+j] += char
		}
	}
	wordSearch += "\n"
	wordSearch += strings.Join(diagonalsUp, "\n")

	diagonalsDown := make([]string, len(lines)*2-1)
	for i, line := range lines {
		for j, char := range strings.Split(line, "") {
			diagonalsDown[len(lines)-1-i+j] += char
		}
	}
	wordSearch += "\n"
	wordSearch += strings.Join(diagonalsDown, "\n")

	return wordSearch
}
