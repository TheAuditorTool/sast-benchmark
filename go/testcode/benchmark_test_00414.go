package testcode

import (
	"io"
	"net/http"
	"os"
)

func BenchmarkTest00414(w http.ResponseWriter, r *http.Request) {
	path := r.URL.Query().Get("path")
	f, err := os.Create(path)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer f.Close()
	io.Copy(f, r.Body)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "uploaded"})
}
