package day05

import (
	"fmt"
	"slices"
	"strings"

	"github.com/mxmlndml/advent-of-code/2024/internal/inputs"
	"github.com/mxmlndml/advent-of-code/2024/internal/shared"
)

func Task1(demo bool) {
	input := inputs.LoadInput(5, 1, demo)
	sections := strings.Split(input, "\n\n")

	rules := pageOrderingRules{}
	rules.load(sections[0])
	updates := strings.Split(sections[1], "\n")

	sum := 0

	for _, update := range updates {
		pages := shared.StringToIntSlice(update, ",")
		isInCorrectOrder := slices.IsSortedFunc(pages, rules.compare)

		if isInCorrectOrder {
			sum += getMiddle(pages)
		}
	}

	fmt.Println(sum)
}
