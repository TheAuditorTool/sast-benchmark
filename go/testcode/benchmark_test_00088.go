package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest00088(w http.ResponseWriter, r *http.Request) {
	_ = r.URL.Query().Get("path")
	tmpFile, err := os.CreateTemp("", "upload-*.dat")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer tmpFile.Close()
	tmpFile.WriteString("data")
	RespondJSON(w, http.StatusCreated, map[string]string{"file": tmpFile.Name()})
}
