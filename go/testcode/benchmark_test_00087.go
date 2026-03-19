package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest00087(w http.ResponseWriter, r *http.Request) {
	filePath := r.URL.Query().Get("path")
	file, err := os.Create(filePath)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer file.Close()
	file.WriteString("data")
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "created"})
}
