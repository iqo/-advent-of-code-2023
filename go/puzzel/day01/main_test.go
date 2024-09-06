package main

import (
	"testing"
	"github.com/stretchr/testify/assert"
)

func testPart1(t *testing.T) {
	assert := assert.New(t)
	var a int = 142
	var test int = partA("day1-input.txt")
	assert.Equal(test, a, "Should be the same.")
}
func testPart2(t *testing.T) {
	assert := assert.New(t)
	var a int = 281
	var test int = partB()
	assert.Equal(test, a, "Should be the same.")
}