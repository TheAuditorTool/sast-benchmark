package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"
)

var benchmarkTest01277UploadDir = "/var/www/uploads"

func BenchmarkTest01277(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(32 << 20)
	file, header, err := r.FormFile("file")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()

	if header.Size > 10<<20 {
		http.Error(w, "file too large", http.StatusRequestEntityTooLarge)
		return
	}

	savePath := filepath.Join(benchmarkTest01277UploadDir, header.Filename)
	dst, err := os.Create(savePath)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()
	io.Copy(dst, file)
	RespondJSON(w, http.StatusOK, map[string]string{"saved": header.Filename})
}
