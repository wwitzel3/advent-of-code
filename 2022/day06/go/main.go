package main

import "os"

func main() {
	content, _ := os.ReadFile("puzzle.input")
	sz, signal_size := len(content), 14

	for i := 0; i < sz; i++ {
		end := i + signal_size
		if end >= sz {
			break
		}
		signal := map[string]bool{}
		for _, c := range content[i:end] {
			signal[string(c)] = true
		}
		if len(signal) == signal_size {
			print(end)
			break
		}
	}
}
