package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"

	"github.com/google/uuid"
)

var benchmarkTest01287UploadDir = "/var/www/uploads"
var benchmarkTest01287AllowedCT = map[string]bool{
	"image/jpeg": true,
	"image/png":  true,
}

func BenchmarkTest01287(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(32 << 20)
	file, header, err := r.FormFile("file")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()

	if header.Size > 5<<20 {
		http.Error(w, "file too large", http.StatusRequestEntityTooLarge)
		return
	}

	buf := make([]byte, 512)
	n, _ := file.Read(buf)
	ct := http.DetectContentType(buf[:n])
	if !benchmarkTest01287AllowedCT[ct] {
		http.Error(w, "type not allowed", http.StatusBadRequest)
		return
	}

	safeName := uuid.New().String()
	dst, err := os.Create(filepath.Join(benchmarkTest01287UploadDir, safeName))
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()
	dst.Write(buf[:n])
	io.Copy(dst, file)
	RespondJSON(w, http.StatusOK, map[string]string{"saved": safeName})
}
