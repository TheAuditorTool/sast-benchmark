package testcode

import (
	"net/http"
	"net/url"
	"path"
)

func BenchmarkTest00548(w http.ResponseWriter, r *http.Request) {
	rawPath := r.URL.Query().Get("path")
	if rawPath == "" {
		rawPath = "/"
	}

	cleanPath := path.Clean("/" + rawPath)

	u := &url.URL{Path: cleanPath}
	http.Redirect(w, r, u.String(), http.StatusFound)
}
