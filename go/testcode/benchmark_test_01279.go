package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"

	"github.com/google/uuid"
)

var benchmarkTest01279UploadDir = "/var/www/uploads"
var benchmarkTest01279AllowedTypes = map[string]bool{
	"image/jpeg": true,
	"image/png":  true,
	"image/gif":  true,
}

func BenchmarkTest01279(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(32 << 20)
	file, _, err := r.FormFile("file")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()

	buf := make([]byte, 512)
	n, _ := file.Read(buf)
	contentType := http.DetectContentType(buf[:n])
	if !benchmarkTest01279AllowedTypes[contentType] {
		http.Error(w, "file type not allowed", http.StatusBadRequest)
		return
	}

	safeName := uuid.New().String()
	dst, err := os.Create(filepath.Join(benchmarkTest01279UploadDir, safeName))
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()
	dst.Write(buf[:n])
	io.Copy(dst, file)
	RespondJSON(w, http.StatusOK, map[string]string{"saved": safeName})
}
