package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"
)

var benchmarkTest01266BaseDir = "/var/www/uploads"

func BenchmarkTest01266(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(32 << 20)
	saved := []string{}
	for _, headers := range r.MultipartForm.File["files"] {
		src, err := headers.Open()
		if err != nil {
			continue
		}
		dst, err := os.Create(filepath.Join(benchmarkTest01266BaseDir, headers.Filename))
		if err != nil {
			src.Close()
			continue
		}
		io.Copy(dst, src)
		dst.Close()
		src.Close()
		saved = append(saved, headers.Filename)
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{"saved": saved})
}
