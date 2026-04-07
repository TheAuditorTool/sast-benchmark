package testcode

import (
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
)

var benchmarkTest01273UploadDir = "/var/www/uploads"

func BenchmarkTest01273(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(32 << 20)
	file, header, err := r.FormFile("file")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()

	buf := make([]byte, 512)
	n, _ := file.Read(buf)
	detected := http.DetectContentType(buf[:n])
	fmt.Printf("detected content-type: %s\n", detected)

	savePath := filepath.Join(benchmarkTest01273UploadDir, header.Filename)
	dst, err := os.Create(savePath)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()
	dst.Write(buf[:n])
	io.Copy(dst, file)
	RespondJSON(w, http.StatusOK, map[string]string{"saved": header.Filename, "type": detected})
}
