package testcode

import (
	"context"
)

type BenchmarkTest00324Request struct {
	ID       string
	Username string
}

type BenchmarkTest00324Response struct {
	Status string
	Error  string
}

func BenchmarkTest00324(ctx context.Context, req *BenchmarkTest00324Request) (*BenchmarkTest00324Response, error) {
	rows, err := DB.QueryContext(ctx, "SELECT * FROM users WHERE id = ?", req.ID)
	if err != nil {
		return &BenchmarkTest00324Response{Error: err.Error()}, nil
	}
	defer rows.Close()
	return &BenchmarkTest00324Response{Status: "ok"}, nil
}
