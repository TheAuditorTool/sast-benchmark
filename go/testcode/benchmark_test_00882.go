package testcode

import (
	"net/http"
	"net/url"
)

func BenchmarkTest00882(w http.ResponseWriter, r *http.Request) {
	rawURL := r.URL.Query().Get("next")
	parsed, err := url.Parse(rawURL)
	if err != nil {
		http.Error(w, "invalid url", http.StatusBadRequest)
		return
	}
	http.Redirect(w, r, parsed.String(), http.StatusFound)
}
