package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"

	"github.com/google/uuid"
)

var benchmarkTest01289UploadDir = "/var/www/uploads"

func BenchmarkTest01289(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(32 << 20)
	file, _, err := r.FormFile("file")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()

	buf := make([]byte, 512)
	n, _ := file.Read(buf)

	if n < 3 || buf[0] != 0xFF || buf[1] != 0xD8 || buf[2] != 0xFF {
		http.Error(w, "only JPEG images accepted", http.StatusBadRequest)
		return
	}

	safeName := uuid.New().String() + ".jpg"
	dst, err := os.Create(filepath.Join(benchmarkTest01289UploadDir, safeName))
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()
	dst.Write(buf[:n])
	io.Copy(dst, file)
	RespondJSON(w, http.StatusOK, map[string]string{"saved": safeName})
}
