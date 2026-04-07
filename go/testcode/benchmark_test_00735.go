package testcode

import (
	"net/http"
)

func BenchmarkTest00735(w http.ResponseWriter, r *http.Request) {
	cookie, err := r.Cookie("proxy_target")
	if err != nil {
		http.Error(w, "missing cookie", http.StatusBadRequest)
		return
	}
	resp, err := http.Get(cookie.Value)
	if err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
