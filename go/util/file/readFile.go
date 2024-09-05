package util

import (
	"log"
	"os"
)

func readFile(filePath string) string {
	file, err := os.ReadFile(filePath)
	if err != nil {
		log.Fatal(err)
	}
	return string(file)
}
