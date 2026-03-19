package testcode

import (
	"net/http"
)

func BenchmarkTest00415(w http.ResponseWriter, r *http.Request) {
	http.StripPrefix("/static/", http.FileServer(http.Dir("./public"))).ServeHTTP(w, r)
}
