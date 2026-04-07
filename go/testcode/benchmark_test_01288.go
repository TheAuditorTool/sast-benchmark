package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"
	"regexp"

	"github.com/google/uuid"
)

var benchmarkTest01288UploadDir = "/var/www/uploads"
var benchmarkTest01288SafeChars = regexp.MustCompile(`[^a-zA-Z0-9._-]`)

func BenchmarkTest01288(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(32 << 20)
	file, header, err := r.FormFile("file")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()

	sanitized := benchmarkTest01288SafeChars.ReplaceAllString(filepath.Base(header.Filename), "")
	safeName := uuid.New().String() + "-" + sanitized

	dst, err := os.Create(filepath.Join(benchmarkTest01288UploadDir, safeName))
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()
	io.Copy(dst, file)
	RespondJSON(w, http.StatusOK, map[string]string{"saved": safeName})
}
