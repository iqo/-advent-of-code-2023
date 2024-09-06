package main

import (
	"fmt"
	"strconv"
	"strings"
	"github.com/iqo/advent-of-code-2023/utility/file"
)

func main() {
	partA("day1-input.txt")
}

func partA(path string) int {
	var numbers string = "0123456789"
	var read string = file.ReadFile(path)
	var hej []string = strings.Split(read, "\n")
	var sum int
	for _, element := range hej {
		var test1 int
		var test2 int
		for i := 0; i < len(element); i++ {
			if strings.ContainsAny(element[i:i+1], numbers) {
				test1, _ = strconv.Atoi(element[i : i+1])
				break
			}
		}
		for i := len(element) - 1; i >= 0; i-- {
			if strings.ContainsAny(element[i:i+1], numbers) {
				test2, _ = strconv.Atoi(element[i : i+1])
				break
			}
		}
		sum += test1*10 + test2
	}
	fmt.Print("input for " + path , ": ", sum, "\n" )
	return sum
}
func partB() int {
	return 1
}