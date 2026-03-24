package testcode

import "net/http"

func BenchmarkTest00331(w http.ResponseWriter, r *http.Request) {
	host := r.URL.Query().Get("host")
	output, err := BenchSvcExecCmdV2(host)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Write(output)
}
