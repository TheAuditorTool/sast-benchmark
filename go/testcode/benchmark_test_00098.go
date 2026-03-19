package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"

	"github.com/google/uuid"
)

func BenchmarkTest00098(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(10 << 20)
	file, _, err := r.FormFile("upload")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()
	safeName := uuid.New().String() + ".dat"
	savePath := filepath.Join("/uploads", safeName)
	dst, err := os.Create(savePath)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()
	io.Copy(dst, file)
	RespondJSON(w, http.StatusCreated, map[string]string{"saved": savePath})
}
