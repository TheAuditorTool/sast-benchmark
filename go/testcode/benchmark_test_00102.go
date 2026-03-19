package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest00102(w http.ResponseWriter, r *http.Request) {
	configName := r.URL.Query().Get("config")
	content, err := os.ReadFile("configs/" + configName + ".yaml")
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	w.Write(content)
}
