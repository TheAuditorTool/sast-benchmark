package testcode

import "net/http"

func benchmarkTest00879RedirectTo(w http.ResponseWriter, r *http.Request, url string) {
	http.Redirect(w, r, url, http.StatusFound)
}

func BenchmarkTest00879(w http.ResponseWriter, r *http.Request) {
	target := r.URL.Query().Get("to")
	benchmarkTest00879RedirectTo(w, r, target)
}
