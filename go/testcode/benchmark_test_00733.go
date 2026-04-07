package testcode

import (
	"encoding/base64"
	"net/http"
)

func BenchmarkTest00733(w http.ResponseWriter, r *http.Request) {
	encoded := r.Header.Get("X-Target-URL")
	decoded, err := base64.StdEncoding.DecodeString(encoded)
	if err != nil {
		http.Error(w, "invalid header", http.StatusBadRequest)
		return
	}
	resp, err := http.Get(string(decoded))
	if err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
