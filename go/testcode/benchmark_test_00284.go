package testcode

import (
	"net/http"
	"os"
)

type benchmarkTest00284FileReq struct {
	Path string
}

func BenchmarkTest00284(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("file")
	req := benchmarkTest00284FileReq{Path: param}
	req.Path = "default.txt"
	content, err := os.ReadFile("/data/" + req.Path)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	w.Write(content)
}
