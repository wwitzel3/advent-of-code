package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
)

func main() {
	file, err := os.Open("puzzle.input")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var totals []int
	value := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		var t = scanner.Text()
		if len(t) == 0 {
			totals = append(totals, value)
			value = 0
			continue
		}
		n, err := strconv.Atoi(t)
		if err != nil {
			panic(err)
		}
		value += n
	}

	sort.Ints(totals)

	fmt.Println(totals[len(totals)-1])

	last3 := 0
	for _, v := range totals[len(totals)-3:] {
		last3 += v
	}

	fmt.Println(last3)
}
