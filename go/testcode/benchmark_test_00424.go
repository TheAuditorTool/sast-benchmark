package testcode

import (
	"fmt"
	"net/http"
	"regexp"
)

func BenchmarkTest00424(w http.ResponseWriter, r *http.Request) {
	filename := r.URL.Query().Get("filename")
	safeName := regexp.MustCompile(`[^a-zA-Z0-9._-]`).ReplaceAllString(filename, "_")
	w.Header().Set("Content-Disposition", fmt.Sprintf("attachment; filename=\"%s\"", safeName))
	w.Header().Set("Content-Type", "application/octet-stream")
	w.Write([]byte("file content"))
}
