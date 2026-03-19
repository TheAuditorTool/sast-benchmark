package testcode

import (
	"net/http"
	"os"
)

type benchmarkTest00286PathReq struct {
	Path string
}

func BenchmarkTest00286(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("file")
	req := benchmarkTest00286PathReq{Path: param}
	content, err := os.ReadFile(req.Path)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	w.Write(content)
}
