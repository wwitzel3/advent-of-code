package main

import (
	"fmt"

	"wwitzel3.id/aoc/solver"
)

type day1 struct {
}

func (d *day1) Solve(input string) string {
	scanner := solver.Scanner(input)
	for scanner.Scan() {
		line := scanner.Text()
		fmt.Println(line)
	}
	return "day1"
}

// Exported
var Solver day1
