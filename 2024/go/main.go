package main

import (
	"fmt"
	"os"
	"path/filepath"
	"plugin"

	s "wwitzel3.id/aoc/solver"
)

// main takes in two arguments from the command-line
// a string and a path to a file
func main() {
	ex, err := os.Executable()
	if err != nil {
		panic(err)
	}
	exPath := filepath.Dir(ex)

	// Print the arguments
	fmt.Println("Arguments:", os.Args)

	// Check if the number of arguments is less than 2
	if len(os.Args) < 2 {
		fmt.Println("Usage: go run main.go <day> <puzzle_input>")
		return
	}

	// Check if the file exists
	if _, err := os.Stat(os.Args[2]); os.IsNotExist(err) {
		fmt.Println("Puzzle input file does not exist")
		return
	}

	// Append .so to first argument
	dayModule := "/days/" + os.Args[1] + ".go.so"
	modulePath := exPath + dayModule

	plugin, err := plugin.Open(modulePath)
	if err != nil {
		fmt.Println("open error")
		return
	}

	symSolver, err := plugin.Lookup("Solver")
	if err != nil {
		fmt.Println("lookup error")
		return
	}

	solver, ok := symSolver.(s.Solver)
	if !ok {
		fmt.Println("unexpected type from module symbol")
		return
	}

	solver.Solve(os.Args[2])
}
