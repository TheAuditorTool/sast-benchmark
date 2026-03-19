package testcode

import (
	"net/http"
	"net/url"
)

func BenchmarkTest00233(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("next")
	if input == "" {
		http.Error(w, "missing next", http.StatusBadRequest)
		return
	}

	u, err := url.Parse(input)
	if err != nil {
		http.Error(w, "invalid url", http.StatusBadRequest)
		return
	}

	http.Redirect(w, r, u.Path, http.StatusFound)
}
