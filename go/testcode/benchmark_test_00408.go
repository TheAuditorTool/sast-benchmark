package testcode

import (
	"archive/zip"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

func BenchmarkTest00408(w http.ResponseWriter, r *http.Request) {
	archivePath := r.URL.Query().Get("archive")
	destDir := "/tmp/extract"
	reader, err := zip.OpenReader(archivePath)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer reader.Close()
	for _, f := range reader.File {
		fullPath := filepath.Join(destDir, f.Name)
		rel, err := filepath.Rel(destDir, fullPath)
		if err != nil || strings.HasPrefix(rel, "..") {
			continue
		}
		if f.FileInfo().IsDir() {
			os.MkdirAll(fullPath, 0755)
			continue
		}
		outFile, err := os.Create(fullPath)
		if err != nil {
			continue
		}
		rc, err := f.Open()
		if err != nil {
			outFile.Close()
			continue
		}
		io.Copy(outFile, rc)
		rc.Close()
		outFile.Close()
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "extracted"})
}
