package file

import (
	"log"
	"os"
)

func ReadFile(filePath string) string {
	file, err := os.ReadFile(filePath)
	if err != nil {
		log.Fatal(err)
	}
	return string(file)
}
