package testcode

import (
	"io"
	"net/http"
	"os"
)

func BenchmarkTest01272(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(32 << 20)
	file, _, err := r.FormFile("file")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()

	tmp, err := os.CreateTemp("", "upload-*")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	tmpPath := tmp.Name()
	io.Copy(tmp, file)
	tmp.Close()

	os.Chmod(tmpPath, 0755)
	RespondJSON(w, http.StatusOK, map[string]string{"path": tmpPath, "status": "queued"})
}
