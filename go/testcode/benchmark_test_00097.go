package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"
)

func BenchmarkTest00097(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(10 << 20)
	file, header, err := r.FormFile("upload")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()
	savePath := filepath.Join("/uploads", header.Filename)
	dst, err := os.Create(savePath)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()
	io.Copy(dst, file)
	RespondJSON(w, http.StatusCreated, map[string]string{"saved": savePath})
}
