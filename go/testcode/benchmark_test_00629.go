package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"
)

func BenchmarkTest00629(w http.ResponseWriter, r *http.Request) {
	if err := r.ParseMultipartForm(50 << 20); err != nil {
		http.Error(w, "failed to parse form", http.StatusBadRequest)
		return
	}

	fileHeaders := r.MultipartForm.File["files"]
	if len(fileHeaders) == 0 {
		http.Error(w, "no files uploaded", http.StatusBadRequest)
		return
	}

	saved := []string{}
	for _, fh := range fileHeaders {
		f, err := fh.Open()
		if err != nil {
			http.Error(w, "failed to open file", http.StatusInternalServerError)
			return
		}

		destPath := filepath.Join("/uploads", fh.Filename)
		dst, err := os.Create(destPath)
		if err != nil {
			f.Close()
			http.Error(w, "failed to save file", http.StatusInternalServerError)
			return
		}

		io.Copy(dst, f)
		dst.Close()
		f.Close()
		saved = append(saved, fh.Filename)
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"status": "uploaded",
		"files":  saved,
		"count":  len(saved),
	})
}
