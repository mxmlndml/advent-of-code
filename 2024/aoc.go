package main

import (
	"errors"
	"fmt"
	"os"
	"strconv"

	"github.com/mxmlndml/advent-of-code/2024/internal"
)

func main() {
	day, task, demo, err := getArguments()
	if err != nil {
		printUsage()
		os.Exit(1)
	}
	internal.Run(day, task, demo)
}

func getArguments() (day int, task int, demo bool, err error) {
	if len(os.Args) < 3 {
		return 0, 0, false, errors.New("wrong usage")
	}

	day, err = strconv.Atoi(os.Args[1])
	if err != nil || day < 1 || day > 25 {
		return 0, 0, false, errors.New("wrong usage")
	}
	task, err = strconv.Atoi(os.Args[2])
	if err != nil || !(task == 1 || task == 2) {
		return 0, 0, false, errors.New("wrong usage")
	}

	demo = false
	if len(os.Args) > 3 {
		demo, err = strconv.ParseBool(os.Args[3])
		if err != nil {
			return 0, 0, false, errors.New("wrong usage")
		}
	}

	return day, task, demo, nil
}

func printUsage() {
	fmt.Printf("Usage: %v \033[4mday\033[0m \033[4mtask\033[0m [demo]\n", os.Args[0])
	fmt.Println("Run Advent of Code implementation on corresponding input file at ./inputs")
	fmt.Println("\n  day\trun implementation for that day (must be between 1 and 25)")
	fmt.Println("  task\trun implementation for that task (must be either 1 or 2)")
	fmt.Println("  demo\trun implementation on demo or final input (must either be true or false)")
}
