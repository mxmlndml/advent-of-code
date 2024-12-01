package internal

import (
	"fmt"
	"log"
	"strings"

	"github.com/mxmlndml/advent-of-code/2024/inputs"
)

func LoadInput(day int, task int, demo bool) string {
	suffix := ""
	if demo {
		suffix = "demo"
	} else {
		suffix = "final"
	}

	path := fmt.Sprintf("day%02d/%1d_%s.txt", day, task, suffix)
	b, err := inputs.Inputs.ReadFile(path)
	if err != nil {
		log.Fatal("failed to read file at '" + path + "'")
	}

	return strings.TrimSpace(string(b))
}

func LoadInputAsLines(day int, task int, demo bool) []string {
	return strings.Split(LoadInput(day, task, demo), "\n")
}
