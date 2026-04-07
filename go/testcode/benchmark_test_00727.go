package testcode

import (
	"net/http"
)

func BenchmarkTest00727(w http.ResponseWriter, r *http.Request) {
	scheme := r.URL.Query().Get("scheme")
	host := r.URL.Query().Get("host")
	path := "/api/data"
	target := scheme + "://" + host + path
	resp, err := http.Get(target)
	if err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
