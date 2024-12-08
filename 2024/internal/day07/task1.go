package day07

import (
	"fmt"
	"log"
	"slices"
	"strconv"
	"strings"

	"github.com/mxmlndml/advent-of-code/2024/internal/inputs"
)

func Task1(demo bool) {
	lines := inputs.LoadInputAsLines(7, 1, demo)

	sum := 0
	for _, line := range lines {
		sum += isSatisfiable(line)
	}
	fmt.Println(sum)
}

func isSatisfiable(line string) int {
	members := strings.Split(line, ": ")
	target, err := strconv.Atoi(members[0])
	if err != nil {
		log.Fatal(err)
	}
	operands := strings.Split(members[1], " ")
	results := make([]int, 0)
	for i, operand := range operands {
		num, err := strconv.Atoi(operand)
		if err != nil {
			log.Fatal(err)
		}

		if i == 0 {
			results = append(results, num)
			continue
		}

		middle := len(results)
		results = slices.Repeat(results, 2)

		for j := range results[:middle] {
			results[j] += num
		}
		for k := range results[middle:] {
			results[middle+k] *= num
		}
	}

	if slices.Contains(results, target) {
		return target
	}
	return 0
}
