package testcode

import (
	"net/http"
)

var benchmarkTest00101PathMap = map[string]string{
	"readme":  "/var/data/docs/README.txt",
	"license": "/var/data/docs/LICENSE.txt",
	"faq":     "/var/data/docs/FAQ.txt",
}

func BenchmarkTest00101(w http.ResponseWriter, r *http.Request) {
	key := r.URL.Query().Get("doc")
	path, ok := benchmarkTest00101PathMap[key]
	if !ok {
		http.Error(w, "document not found", http.StatusNotFound)
		return
	}
	http.ServeFile(w, r, path)
}
