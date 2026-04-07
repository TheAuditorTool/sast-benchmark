package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

var benchmarkTest01274UploadDir = "/var/www/uploads"

func BenchmarkTest01274(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(32 << 20)
	file, header, err := r.FormFile("file")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()

	name := header.Filename
	if !strings.HasSuffix(name, ".jpg") {
		http.Error(w, "only jpg allowed", http.StatusBadRequest)
		return
	}

	savePath := filepath.Join(benchmarkTest01274UploadDir, name)
	dst, err := os.Create(savePath)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()
	io.Copy(dst, file)
	RespondJSON(w, http.StatusOK, map[string]string{"saved": name})
}
