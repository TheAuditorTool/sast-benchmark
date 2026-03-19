package testcode

import (
	"io"
	"net/http"
)

func BenchmarkTest00139(w http.ResponseWriter, r *http.Request) {
	endpoint := r.FormValue("endpoint")
	resp, err := http.Get(endpoint)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	RespondJSON(w, http.StatusOK, map[string]string{"result": string(body)})
}
