package testcode

import (
	"net/http"
	"path/filepath"
)

func BenchmarkTest00898(w http.ResponseWriter, r *http.Request) {
	page := r.URL.Query().Get("page")
	safePage := "/" + filepath.Base(page)
	http.Redirect(w, r, safePage, http.StatusFound)
}
