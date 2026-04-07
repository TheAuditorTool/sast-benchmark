package testcode

import (
	"net/http"
	"os"
	"strings"
)

func BenchmarkTest01147(w http.ResponseWriter, r *http.Request) {
	filename := strings.TrimPrefix(r.URL.Path, "/files/")
	if filename == "" {
		http.Error(w, "missing filename", http.StatusBadRequest)
		return
	}

	data, err := os.ReadFile("/var/uploads/" + filename)
	if err != nil {
		http.Error(w, "file not found", http.StatusNotFound)
		return
	}

	w.Header().Set("Content-Disposition", "attachment; filename="+filename)
	w.Write(data)
}
