package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00423(w http.ResponseWriter, r *http.Request) {
	filename := r.URL.Query().Get("filename")
	w.Header().Set("Content-Disposition", fmt.Sprintf("attachment; filename=\"%s\"", filename))
	w.Header().Set("Content-Type", "application/octet-stream")
	w.Write([]byte("file content"))
}
