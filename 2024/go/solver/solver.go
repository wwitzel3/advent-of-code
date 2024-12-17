package solver

type aocSolver struct {
}

func (a *aocSolver) Solve(s string) string {
	return s
}

func New() Solver {
	return &aocSolver{}
}

type Solver interface {
	Solve(string) string
}
