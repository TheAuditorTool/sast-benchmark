package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest00110(w http.ResponseWriter, r *http.Request) {
	backupName := r.URL.Query().Get("backup")
	srcPath := "/var/data/" + backupName
	dstPath := "/var/backups/" + backupName
	data, err := os.ReadFile(srcPath)
	if err != nil {
		http.Error(w, "source not found", http.StatusNotFound)
		return
	}
	os.WriteFile(dstPath, data, 0644)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "backed up"})
}
