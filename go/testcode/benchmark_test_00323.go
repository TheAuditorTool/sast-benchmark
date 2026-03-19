package testcode

import (
	"context"
	"os/exec"
)

type BenchmarkTest00323Request struct {
	Command string
	Args    []string
}

type BenchmarkTest00323Response struct {
	Output string
	Error  string
}

func BenchmarkTest00323(ctx context.Context, req *BenchmarkTest00323Request) (*BenchmarkTest00323Response, error) {
	cmd := exec.CommandContext(ctx, req.Command, req.Args...)
	out, err := cmd.Output()
	if err != nil {
		return &BenchmarkTest00323Response{Error: err.Error()}, nil
	}
	return &BenchmarkTest00323Response{Output: string(out)}, nil
}
