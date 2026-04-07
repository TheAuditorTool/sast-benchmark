package testcode

import (
	"io"
	"net/http"
	"os"
	"strings"
)

var benchmarkTest01268UploadDir = "/var/www/uploads/"

func BenchmarkTest01268(w http.ResponseWriter, r *http.Request) {
	cd := r.Header.Get("Content-Disposition")
	filename := ""
	for _, part := range strings.Split(cd, ";") {
		part = strings.TrimSpace(part)
		if strings.HasPrefix(part, "filename=") {
			filename = strings.Trim(strings.TrimPrefix(part, "filename="), `"`)
		}
	}
	if filename == "" {
		http.Error(w, "filename required", http.StatusBadRequest)
		return
	}

	dst, err := os.Create(benchmarkTest01268UploadDir + filename)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()
	io.Copy(dst, r.Body)
	RespondJSON(w, http.StatusOK, map[string]string{"file": filename})
}
