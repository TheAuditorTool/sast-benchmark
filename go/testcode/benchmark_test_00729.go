package testcode

import (
	"context"
	"net/http"
)

type benchmarkTest00729CtxKey string

func benchmarkTest00729Fetch(ctx context.Context) error {
	url, _ := ctx.Value(benchmarkTest00729CtxKey("target_url")).(string)
	resp, err := http.Get(url)
	if err != nil {
		return err
	}
	resp.Body.Close()
	return nil
}

func BenchmarkTest00729(w http.ResponseWriter, r *http.Request) {
	targetURL := r.URL.Query().Get("url")
	ctx := context.WithValue(r.Context(), benchmarkTest00729CtxKey("target_url"), targetURL)
	if err := benchmarkTest00729Fetch(ctx); err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
