package testcode

import (
	"net/http"
	"os"
	"path/filepath"

	"github.com/gorilla/mux"
)

func BenchmarkTest00317(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	filename := vars["file"]
	safeName := filepath.Base(filename)
	data, err := os.ReadFile("/uploads/" + safeName)
	if err != nil {
		http.Error(w, err.Error(), http.StatusNotFound)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"content": string(data)})
}
