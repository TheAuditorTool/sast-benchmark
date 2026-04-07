package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"

	"github.com/google/uuid"
)

var benchmarkTest01292UploadDir = "/var/www/uploads"

func BenchmarkTest01292(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(32 << 20)
	file, _, err := r.FormFile("file")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()

	safeName := uuid.New().String()
	dst, err := os.Create(filepath.Join(benchmarkTest01292UploadDir, safeName))
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()

	limited := io.LimitReader(file, 10<<20)
	buf := make([]byte, 512)
	n, _ := file.Read(buf)
	detectedType := http.DetectContentType(buf[:n])
	dst.Write(buf[:n])
	io.Copy(dst, limited)

	w.Header().Set("X-Detected-Type", detectedType)
	RespondJSON(w, http.StatusOK, map[string]string{"saved": safeName, "type": detectedType})
}
