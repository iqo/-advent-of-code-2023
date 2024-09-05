package main
import(
"testing"
"github.com/stretchr/testify/assert"
)
func testPart1(t *testing.T) {
	assert := assert.New(t)

	var a string = "Hello"
	var b string = "Hello"
  
	assert.Equal(a, b, "The two words should be the same.")
}