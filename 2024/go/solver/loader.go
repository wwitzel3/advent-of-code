package solver

import (
	"bufio"
	"os"
)

func RawContents(path string) []byte {
	raw, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	return raw
}

func Scanner(path string) *bufio.Scanner {
	file, err := os.Open(path)
	if err != nil {
		panic(err)
	}
	return bufio.NewScanner(file)
}
