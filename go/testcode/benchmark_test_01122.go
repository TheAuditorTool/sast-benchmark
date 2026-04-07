package testcode

import (
	"net/http"
)

func BenchmarkTest01122(w http.ResponseWriter, r *http.Request) {
	token := r.Header.Get("X-Session")
	if token == "" {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "authenticated"})
}
