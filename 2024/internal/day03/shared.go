package day03

import (
	"strconv"
	"strings"
)

func mulIfPossible(memory string) int {
	comma := strings.Index(memory, ",")
	if comma == -1 || comma > 7 {
		return 0
	}

	closingParenthesis := strings.Index(memory, ")")
	if closingParenthesis == -1 || closingParenthesis > 11 {
		return 0
	}

	a, err := strconv.Atoi(memory[4:comma])
	if err != nil {
		return 0
	}
	b, err := strconv.Atoi(memory[comma+1 : closingParenthesis])
	if err != nil {
		return 0
	}

	return a * b
}
