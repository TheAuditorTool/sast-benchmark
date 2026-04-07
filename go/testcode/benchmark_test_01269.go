package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"
)

func BenchmarkTest01269(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(32 << 20)
	file, header, err := r.FormFile("file")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()

	dir := r.FormValue("dir")
	savePath := filepath.Join(dir, header.Filename)

	dst, err := os.Create(savePath)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()
	io.Copy(dst, file)
	RespondJSON(w, http.StatusOK, map[string]string{"saved": savePath})
}
