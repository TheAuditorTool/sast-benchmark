package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"

	"github.com/google/uuid"
)

var benchmarkTest01290UploadDir = "/var/www/uploads"
var benchmarkTest01290AllowedExts = map[string]string{
	".jpg":  ".jpg",
	".jpeg": ".jpg",
	".png":  ".png",
	".gif":  ".gif",
}

func BenchmarkTest01290(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(32 << 20)
	file, header, err := r.FormFile("file")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()

	ext := strings.ToLower(filepath.Ext(header.Filename))
	safeExt, ok := benchmarkTest01290AllowedExts[ext]
	if !ok {
		http.Error(w, "extension not allowed", http.StatusBadRequest)
		return
	}

	safeName := uuid.New().String() + safeExt
	dst, err := os.Create(filepath.Join(benchmarkTest01290UploadDir, safeName))
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()
	io.Copy(dst, file)
	RespondJSON(w, http.StatusOK, map[string]string{"saved": safeName})
}
