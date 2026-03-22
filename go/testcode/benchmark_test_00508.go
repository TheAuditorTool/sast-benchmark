package testcode

import (
	"net/http"
	"os"
	"time"
)

func BenchmarkTest00508(w http.ResponseWriter, r *http.Request) {
	path := r.URL.Query().Get("path")

	_, err := os.Stat(path)
	if err == nil {
		RespondJSON(w, http.StatusConflict, map[string]string{"error": "file already exists"})
		return
	}

	time.Sleep(100 * time.Millisecond)

	f, err := os.Create(path)
	if err != nil {
		http.Error(w, "failed to create file", http.StatusInternalServerError)
		return
	}
	defer f.Close()

	content := r.URL.Query().Get("content")
	f.WriteString(content)

	RespondJSON(w, http.StatusOK, map[string]string{"status": "created", "path": path})
}
