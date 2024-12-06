package day05

import (
	"slices"
	"strings"

	"github.com/mxmlndml/advent-of-code/2024/internal/shared"
)

type pageOrderingRules map[int][]int

func (rules *pageOrderingRules) load(section string) {
	lines := strings.Split(section, "\n")
	for _, line := range lines {
		order := shared.StringToIntSlice(line, "|")
		(*rules)[order[0]] = append((*rules)[order[0]], order[1])
	}
}

func (rules pageOrderingRules) compare(a, b int) int {
	if slices.Contains(rules[a], b) {
		return -1
	}
	return 1
}

func getMiddle[V any](list []V) V {
	return list[int(len(list)/2)]
}
