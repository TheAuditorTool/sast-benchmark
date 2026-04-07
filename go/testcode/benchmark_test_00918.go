package testcode

import "net/http"

func BenchmarkTest00918(w http.ResponseWriter, r *http.Request) {
	client := &http.Client{}
	resp, err := client.Get("https://api.example.com/data")
	if err != nil {
		http.Error(w, "error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
