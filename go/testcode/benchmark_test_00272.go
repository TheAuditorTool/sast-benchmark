package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest00272(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("file")
	m := make(map[string]string)
	m["upload"] = param
	path := m["default"]
	content, err := os.ReadFile("/data/" + path)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	w.Write(content)
}
