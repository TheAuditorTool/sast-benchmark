package testcode

import (
	"io"
	"net/http"
)

func BenchmarkTest00133(w http.ResponseWriter, r *http.Request) {
	userURL := r.URL.Query().Get("endpoint")
	req, err := http.NewRequest("GET", userURL, nil)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	RespondJSON(w, http.StatusOK, map[string]string{"result": string(body)})
}
