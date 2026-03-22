package testcode

import (
	"context"
	"net/http"
)

type benchmarkTest00480Key struct{}

func BenchmarkTest00480(w http.ResponseWriter, r *http.Request) {
	userData := r.Header.Get("X-User-Data")
	ctx := context.WithValue(r.Context(), benchmarkTest00480Key{}, userData)
	benchmarkTest00480Downstream(w, r.WithContext(ctx))
}

func benchmarkTest00480Downstream(w http.ResponseWriter, r *http.Request) {
	userData := r.Context().Value(benchmarkTest00480Key{}).(string)
	RespondJSON(w, http.StatusOK, map[string]string{"user_data": userData})
}
