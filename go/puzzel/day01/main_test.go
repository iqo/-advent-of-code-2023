package main

import (
	"testing"
	"github.com/stretchr/testify/assert"
)

func testPart1(t *testing.T) {
	assert := assert.New(t)
	var a string = "Hello"
	var test string = partA()
	assert.Equal(test, a, "The two words should be the same.")
}
