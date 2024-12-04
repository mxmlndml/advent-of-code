package day03

import (
	"fmt"
	"strings"

	"github.com/mxmlndml/advent-of-code/2024/internal/inputs"
)

func Task2(demo bool) {
	var memory string
	if demo {
		memory = inputs.LoadInput(3, 2, true)
	} else {
		memory = inputs.LoadInput(3, 1, false)
	}

	sum := 0
	enabled := true

	for {
		i := strings.Index(memory, "mul(")
		j := -1
		if enabled {
			j = strings.Index(memory, "don't()")
		} else {
			j = strings.Index(memory, "do()")
		}

		if i == -1 || (!enabled && j == -1) {
			break
		}

		if !enabled {
			memory = memory[j+1:]
			enabled = true
			continue
		}

		if i < j || j == -1 {
			sum += mulIfPossible(memory[i:])
			memory = memory[i+1:]
			continue
		}

		memory = memory[j+1:]
		enabled = false
	}

	fmt.Println(sum)
}
