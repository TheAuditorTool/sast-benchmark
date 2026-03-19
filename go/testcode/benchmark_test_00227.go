package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00227(w http.ResponseWriter, r *http.Request) {
	userURL := r.URL.Query().Get("url")
	if userURL == "" {
		http.Error(w, "missing url", http.StatusBadRequest)
		return
	}

	w.Header().Set("Content-Type", "text/html")
	w.WriteHeader(http.StatusOK)
	fmt.Fprintf(w, "<html><head><meta http-equiv='refresh' content='0;url=%s'></head><body>Redirecting...</body></html>", userURL)
}
