package testcode

import (
	"io"
	"net/http"
	"os"
)

func BenchmarkTest00085(w http.ResponseWriter, r *http.Request) {
	savePath := r.URL.Query().Get("path")
	body, _ := io.ReadAll(r.Body)
	err := os.WriteFile(savePath, body, 0644)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "saved"})
}
