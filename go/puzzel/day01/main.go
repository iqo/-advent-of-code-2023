package main

import (
	"fmt"

	"github.com/iqo/advent-of-code-2023/utility/file"
)

func main() {
	partA()
}
func partA() string {
	read := file.ReadFile("day1-input-test-data.txt")
	fmt.Print(read)
	return read
}
