package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"

	"github.com/google/uuid"
)

var benchmarkTest01281UploadDir = "/var/www/uploads"

func BenchmarkTest01281(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(32 << 20)
	file, _, err := r.FormFile("file")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()

	safeName := uuid.New().String()
	dst, err := os.Create(filepath.Join(benchmarkTest01281UploadDir, safeName))
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()

	limited := io.LimitReader(file, 10<<20)
	io.Copy(dst, limited)
	RespondJSON(w, http.StatusOK, map[string]string{"saved": safeName})
}
