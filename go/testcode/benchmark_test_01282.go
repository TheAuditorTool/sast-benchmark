package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"

	"github.com/google/uuid"
)

var benchmarkTest01282UploadDir = "/var/www/uploads"
var benchmarkTest01282AllowedMIME = map[string]bool{
	"image/jpeg":      true,
	"image/png":       true,
	"application/pdf": true,
}

func BenchmarkTest01282(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(32 << 20)
	file, _, err := r.FormFile("file")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()

	magic := make([]byte, 512)
	n, _ := file.Read(magic)
	mimeType := http.DetectContentType(magic[:n])
	if !benchmarkTest01282AllowedMIME[mimeType] {
		http.Error(w, "unsupported file type", http.StatusBadRequest)
		return
	}

	safeName := uuid.New().String()
	dst, err := os.Create(filepath.Join(benchmarkTest01282UploadDir, safeName))
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()
	dst.Write(magic[:n])
	io.Copy(dst, file)
	RespondJSON(w, http.StatusOK, map[string]string{"saved": safeName})
}
