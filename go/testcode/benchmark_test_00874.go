package testcode

import "net/http"

func BenchmarkTest00874(w http.ResponseWriter, r *http.Request) {
	referer := r.Header.Get("Referer")
	if referer == "" {
		referer = "/"
	}
	http.Redirect(w, r, referer, http.StatusMovedPermanently)
}
