package testcode

import (
	"net/http"
	"net/url"
)

func BenchmarkTest01296(w http.ResponseWriter, r *http.Request) {
	rawURL := r.FormValue("url")
	parsed, err := url.Parse(rawURL)
	if err != nil {
		http.Error(w, "bad url", http.StatusBadRequest)
		return
	}
	http.Redirect(w, r, parsed.String(), http.StatusFound)
}
