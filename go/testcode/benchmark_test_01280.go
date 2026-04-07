package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"
)

var benchmarkTest01280UploadDir = "/var/www/uploads"

func BenchmarkTest01280(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(32 << 20)
	file, header, err := r.FormFile("file")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()

	safeName := filepath.Base(header.Filename)
	savePath := filepath.Join(benchmarkTest01280UploadDir, safeName)
	dst, err := os.Create(savePath)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()
	io.Copy(dst, file)
	RespondJSON(w, http.StatusOK, map[string]string{"saved": safeName})
}
