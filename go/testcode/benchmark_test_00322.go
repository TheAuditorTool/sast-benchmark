package testcode

import (
	"context"
	"fmt"
)

type BenchmarkTest00322Request struct {
	ID       string
	Username string
}

type BenchmarkTest00322Response struct {
	Status string
	Error  string
}

func BenchmarkTest00322(ctx context.Context, req *BenchmarkTest00322Request) (*BenchmarkTest00322Response, error) {
	query := fmt.Sprintf("SELECT * FROM users WHERE id = %s", req.ID)
	rows, err := DB.QueryContext(ctx, query)
	if err != nil {
		return &BenchmarkTest00322Response{Error: err.Error()}, nil
	}
	defer rows.Close()
	return &BenchmarkTest00322Response{Status: "ok"}, nil
}
