package main

import (
	"fmt"
	"strings"

	"github.com/iqo/advent-of-code-2023/utility/file"
)
var input string

func main() {
	partA()
}
func partA() string {
	var read string = file.ReadFile("day1-input-test-data.txt")
	var hej []string = strings.Split(read, "\n")
	fmt.Print(hej)
	return read
}
