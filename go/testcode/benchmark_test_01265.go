package testcode

import (
	"io"
	"net/http"
	"os"
)

func BenchmarkTest01265(w http.ResponseWriter, r *http.Request) {
	path := r.URL.Query().Get("path")
	if path == "" {
		http.Error(w, "path required", http.StatusBadRequest)
		return
	}

	dst, err := os.Create(path)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()

	io.Copy(dst, r.Body)
	RespondJSON(w, http.StatusOK, map[string]string{"written": path})
}
