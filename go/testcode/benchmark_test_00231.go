package testcode

import (
	"net/http"
)

func BenchmarkTest00231(w http.ResponseWriter, r *http.Request) {
	_ = r.URL.Query().Get("next")

	http.Redirect(w, r, "/dashboard", http.StatusFound)
}
