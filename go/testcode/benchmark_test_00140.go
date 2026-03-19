package testcode

import (
	"io"
	"net/http"
)

func BenchmarkTest00140(w http.ResponseWriter, r *http.Request) {
	cookie, err := r.Cookie("proxy_target")
	if err != nil {
		http.Error(w, "missing cookie", http.StatusBadRequest)
		return
	}
	resp, fetchErr := http.Get(cookie.Value)
	if fetchErr != nil {
		http.Error(w, fetchErr.Error(), http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	RespondJSON(w, http.StatusOK, map[string]string{"proxied": string(body)})
}
