package testcode

import (
	"net/http"

	"github.com/go-chi/chi/v5"
)

func BenchmarkTest00736(w http.ResponseWriter, r *http.Request) {
	endpoint := chi.URLParam(r, "endpoint")
	target := "http://" + endpoint
	resp, err := http.Get(target)
	if err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
