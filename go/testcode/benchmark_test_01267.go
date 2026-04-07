package testcode

import (
	"archive/zip"
	"io"
	"net/http"
	"os"
)

var benchmarkTest01267ExtractDir = "/var/www/uploads/extracted"

func BenchmarkTest01267(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(32 << 20)
	file, header, err := r.FormFile("archive")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()

	tmp, _ := os.CreateTemp("", "upload-*.zip")
	io.Copy(tmp, file)
	tmp.Close()
	defer os.Remove(tmp.Name())

	zr, err := zip.OpenReader(tmp.Name())
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer zr.Close()

	for _, entry := range zr.File {
		outPath := benchmarkTest01267ExtractDir + "/" + entry.Name
		rc, _ := entry.Open()
		dst, _ := os.Create(outPath)
		io.Copy(dst, rc)
		dst.Close()
		rc.Close()
	}
	RespondJSON(w, http.StatusOK, map[string]string{"archive": header.Filename})
}
