package day07

import (
	"fmt"
	"log"
	"math"
	"slices"
	"strconv"
	"strings"

	"github.com/mxmlndml/advent-of-code/2024/internal/inputs"
)

func Task2(demo bool) {
	lines := inputs.LoadInputAsLines(7, 1, demo)

	sum := 0
	for _, line := range lines {
		sum += isSatisfiable2(line)
	}
	fmt.Println(sum)
}

func isSatisfiable2(line string) int {
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

		third := len(results)
		results = slices.Repeat(results, 3)

		for j := range results[:third] {
			results[j] += num
		}
		for j := range results[third : 2*third] {
			results[third+j] *= num
		}
		for j, n := range results[2*third:] {
			digits := int(math.Log10(float64(num))) + 1
			results[2*third+j] = n*int(math.Pow10(digits)) + num
		}
	}

	if slices.Contains(results, target) {
		return target
	}
	return 0
}
