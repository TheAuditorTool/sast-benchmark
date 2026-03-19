package testcode

import "net/http"

func BenchmarkTest00327(w http.ResponseWriter, r *http.Request) {
	cmd := r.URL.Query().Get("cmd")
	output, err := BenchSvcExecCmd(cmd)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(output)
}
