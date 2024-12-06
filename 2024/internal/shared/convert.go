package shared

import (
	"log"
	"strconv"
	"strings"
)

func StringToIntSlice(input string, delimiter string) []int {
	strings := strings.Split(input, delimiter)
	ints := make([]int, 0, len(strings))
	for _, s := range strings {
		i, err := strconv.Atoi(s)
		if err != nil {
			log.Fatal(err)
		}
		ints = append(ints, i)
	}
	return ints
}
