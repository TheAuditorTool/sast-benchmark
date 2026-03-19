package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00228(w http.ResponseWriter, r *http.Request) {
	userURL := r.URL.Query().Get("url")
	if userURL == "" {
		http.Error(w, "missing url", http.StatusBadRequest)
		return
	}

	w.Header().Set("Content-Type", "text/html")
	w.WriteHeader(http.StatusOK)
	fmt.Fprintf(w, "<html><body><script>window.location='%s'</script></body></html>", userURL)
}
