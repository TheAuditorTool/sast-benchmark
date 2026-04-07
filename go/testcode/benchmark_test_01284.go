package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

var benchmarkTest01284BaseDir = "/var/www/uploads"

func BenchmarkTest01284(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(32 << 20)
	file, header, err := r.FormFile("file")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()

	candidate := filepath.Join(benchmarkTest01284BaseDir, header.Filename)
	cleaned := filepath.Clean(candidate)
	if !strings.HasPrefix(cleaned, benchmarkTest01284BaseDir+string(filepath.Separator)) {
		http.Error(w, "invalid path", http.StatusBadRequest)
		return
	}

	dst, err := os.Create(cleaned)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()
	io.Copy(dst, file)
	RespondJSON(w, http.StatusOK, map[string]string{"saved": filepath.Base(cleaned)})
}
