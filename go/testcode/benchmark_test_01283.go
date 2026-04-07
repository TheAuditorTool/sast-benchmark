package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"

	"github.com/google/uuid"
)

var benchmarkTest01283UploadDir = "/var/www/uploads"
var benchmarkTest01283AllowedExts = map[string]bool{
	".jpg":  true,
	".jpeg": true,
	".png":  true,
	".pdf":  true,
}

func BenchmarkTest01283(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(32 << 20)
	file, header, err := r.FormFile("file")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()

	ext := strings.ToLower(filepath.Ext(header.Filename))
	if !benchmarkTest01283AllowedExts[ext] {
		http.Error(w, "extension not allowed", http.StatusBadRequest)
		return
	}

	safeName := uuid.New().String() + ext
	dst, err := os.Create(filepath.Join(benchmarkTest01283UploadDir, safeName))
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()
	io.Copy(dst, file)
	RespondJSON(w, http.StatusOK, map[string]string{"saved": safeName})
}
