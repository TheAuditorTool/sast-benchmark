package testcode

import (
	"context"
	"os"
	"path/filepath"
)

type BenchmarkTest00325Request struct {
	Path string
}

type BenchmarkTest00325Response struct {
	Content string
	Error   string
}

func BenchmarkTest00325(ctx context.Context, req *BenchmarkTest00325Request) (*BenchmarkTest00325Response, error) {
	safeName := filepath.Base(req.Path)
	data, err := os.ReadFile("/data/" + safeName)
	if err != nil {
		return &BenchmarkTest00325Response{Error: err.Error()}, nil
	}
	return &BenchmarkTest00325Response{Content: string(data)}, nil
}
