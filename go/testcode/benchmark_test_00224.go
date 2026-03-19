package testcode

import (
	"net/http"
)

func BenchmarkTest00224(w http.ResponseWriter, r *http.Request) {
	redirectURL := r.URL.Query().Get("redirect_url")
	if redirectURL == "" {
		http.Error(w, "missing redirect_url", http.StatusBadRequest)
		return
	}

	http.Redirect(w, r, redirectURL, http.StatusMovedPermanently)
}
