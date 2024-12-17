package main

import "fmt"

type solver struct {
}

func (s *solver) Solve(input string) string {
	fmt.Println("Solving day01", input)
	return "day01"
}

// Exported
var Solver solver
