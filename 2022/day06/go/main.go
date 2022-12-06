package main

import "os"

func main() {
	content, _ := os.ReadFile("puzzle.input")
	sz := len(content)
	signal_size := 14

	for i := 0; i < sz; i++ {
		end := i + signal_size
		if end >= sz {
			break
		}
		s := content[i:end]
		signal := map[string]bool{}
		for _, c := range s {
			signal[string(c)] = true
		}
		if len(signal) == signal_size {
			print(end)
			break
		}
	}
}
