package testcode

import (
	"archive/zip"
	"bytes"
	"io"
	"net/http"
	"os"
	"path/filepath"
)

const benchmarkTest00627ExtractDir = "/var/app/extracted"

func BenchmarkTest00627(w http.ResponseWriter, r *http.Request) {
	file, header, err := r.FormFile("archive")
	if err != nil {
		http.Error(w, "no archive uploaded", http.StatusBadRequest)
		return
	}
	defer file.Close()

	data, err := io.ReadAll(file)
	if err != nil {
		http.Error(w, "failed to read archive", http.StatusInternalServerError)
		return
	}

	zr, err := zip.NewReader(bytes.NewReader(data), int64(len(data)))
	if err != nil {
		http.Error(w, "invalid zip archive", http.StatusBadRequest)
		return
	}

	extracted := []string{}
	for _, f := range zr.File {
		destPath := filepath.Join(benchmarkTest00627ExtractDir, f.Name)

		if f.FileInfo().IsDir() {
			os.MkdirAll(destPath, 0755)
			continue
		}

		dst, err := os.Create(destPath)
		if err != nil {
			http.Error(w, "extraction failed", http.StatusInternalServerError)
			return
		}

		rc, err := f.Open()
		if err != nil {
			dst.Close()
			http.Error(w, "extraction failed", http.StatusInternalServerError)
			return
		}

		io.Copy(dst, rc)
		rc.Close()
		dst.Close()
		extracted = append(extracted, f.Name)
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"status":    "extracted",
		"archive":   header.Filename,
		"files":     extracted,
		"count":     len(extracted),
	})
}
