package testcode

import (
	"net/http"
	"path/filepath"
)

func BenchmarkTest00268(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("file")
	name := param
	name = filepath.Base("static/default.css")
	http.ServeFile(w, r, "./public/"+name)
}
