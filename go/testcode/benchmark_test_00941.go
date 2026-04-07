package testcode

import (
	"context"
	"encoding/json"
	"net/http"
)

type benchmarkTest00941CtxKey struct{}

func BenchmarkTest00941(w http.ResponseWriter, r *http.Request) {
	var payload interface{}
	json.NewDecoder(r.Body).Decode(&payload)
	ctx := context.WithValue(r.Context(), benchmarkTest00941CtxKey{}, payload)
	data := ctx.Value(benchmarkTest00941CtxKey{})
	RespondJSON(w, http.StatusOK, map[string]interface{}{"data": data})
}
