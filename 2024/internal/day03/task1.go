package day03

import (
	"fmt"
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
