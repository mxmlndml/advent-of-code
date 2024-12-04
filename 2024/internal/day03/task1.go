package day03

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/mxmlndml/advent-of-code/2024/internal/inputs"
)

func Task1(demo bool) {
	memory := inputs.LoadInput(3, 1, demo)

	sum := 0

	for {
		i := strings.Index(memory, "mul(")
		if i == -1 {
			break
		}
		sum += mulIfPossible(memory[i:])
		memory = memory[i+1:]
	}

	fmt.Println(sum)
}

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
