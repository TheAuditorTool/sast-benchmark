package testcode

import (
	"archive/zip"
	"io"
	"net/http"
	"os"
	"path/filepath"
)

func BenchmarkTest00407(w http.ResponseWriter, r *http.Request) {
	archivePath := r.URL.Query().Get("archive")
	reader, err := zip.OpenReader(archivePath)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer reader.Close()
	for _, f := range reader.File {
		path := filepath.Join("/tmp/extract", f.Name)
		outFile, _ := os.Create(path)
		rc, _ := f.Open()
		io.Copy(outFile, rc)
		rc.Close()
		outFile.Close()
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "extracted"})
}
