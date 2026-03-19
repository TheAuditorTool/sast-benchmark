package testcode

import "net/http"

func BenchmarkTest00332(w http.ResponseWriter, r *http.Request) {
	file := r.URL.Query().Get("file")
	content, err := BenchSvcReadPathSafe(file)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	w.Write(content)
}
