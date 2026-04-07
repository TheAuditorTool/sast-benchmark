package testcode

import (
	"net/http"
)

func BenchmarkTest00728(w http.ResponseWriter, r *http.Request) {
	serviceKey := r.URL.Query().Get("service")
	var endpoint string
	err := DB.QueryRow("SELECT url FROM service_registry WHERE key = ?", serviceKey).Scan(&endpoint)
	if err != nil {
		http.Error(w, "service not found", http.StatusNotFound)
		return
	}
	resp, err := http.Get(endpoint)
	if err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
