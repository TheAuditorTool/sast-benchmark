package testcode

import (
	"net/http"
	"os"
	"path/filepath"
)

func BenchmarkTest00100(w http.ResponseWriter, r *http.Request) {
	logName := r.URL.Query().Get("log")
	logPath := filepath.Join("/var/log/app", logName)
	f, err := os.OpenFile(logPath, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer f.Close()
	f.WriteString("log entry\n")
	RespondJSON(w, http.StatusOK, map[string]string{"status": "logged"})
}
