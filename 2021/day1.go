package main

import (
	"io/ioutil"
	"strings"
)

func main() {
	file, err := ioutil.ReadFile("day1.test.input")
	if err != nil {
		panic(err)
	}

	input := string(file)
	inputs := strings.Split(input, "\n")

	for _, v := range inputs {
		print(v)
	}
}
