package testcode

import "net/http"

func BenchmarkTest00888(w http.ResponseWriter, r *http.Request) {
	http.Redirect(w, r, "/dashboard", http.StatusFound)
}
