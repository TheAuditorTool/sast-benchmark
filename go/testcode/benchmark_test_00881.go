package testcode

import (
	"context"
	"net/http"
)

type benchmarkTest00881Key struct{}

func BenchmarkTest00881(w http.ResponseWriter, r *http.Request) {
	returnURL := r.URL.Query().Get("return")
	ctx := context.WithValue(r.Context(), benchmarkTest00881Key{}, returnURL)
	url := ctx.Value(benchmarkTest00881Key{}).(string)
	http.Redirect(w, r, url, http.StatusFound)
}
