package testcode

import "net/http"

func BenchmarkTest00930(w http.ResponseWriter, r *http.Request) {
	resp, err := http.Get("https://api.example.com/status")
	if err != nil {
		http.Error(w, "error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
