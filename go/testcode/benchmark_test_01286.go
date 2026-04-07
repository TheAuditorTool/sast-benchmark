package testcode

import (
	"io"
	"net/http"
	"os"
)

var benchmarkTest01286UploadDir = "/var/www/uploads"

func BenchmarkTest01286(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(32 << 20)
	file, _, err := r.FormFile("file")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()

	dst, err := os.CreateTemp(benchmarkTest01286UploadDir, "upload-*")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()

	io.Copy(dst, file)
	RespondJSON(w, http.StatusOK, map[string]string{"saved": dst.Name()})
}
