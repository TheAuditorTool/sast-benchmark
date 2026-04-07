package testcode

import (
	"archive/zip"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

var benchmarkTest01285ExtractDir = "/var/www/uploads/extracted"

func BenchmarkTest01285(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(32 << 20)
	file, _, err := r.FormFile("archive")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()

	tmp, _ := os.CreateTemp("", "zip-*.zip")
	io.Copy(tmp, file)
	tmp.Close()
	defer os.Remove(tmp.Name())

	zr, err := zip.OpenReader(tmp.Name())
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer zr.Close()

	for _, entry := range zr.File {
		cleanPath := filepath.Clean(entry.Name)
		if strings.HasPrefix(cleanPath, "..") {
			continue
		}
		outPath := filepath.Join(benchmarkTest01285ExtractDir, cleanPath)
		rc, _ := entry.Open()
		dst, _ := os.Create(outPath)
		io.Copy(dst, rc)
		dst.Close()
		rc.Close()
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "extracted"})
}
