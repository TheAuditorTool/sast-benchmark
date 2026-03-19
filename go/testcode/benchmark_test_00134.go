package testcode

import (
	"io"
	"net/http"
)

func BenchmarkTest00134(w http.ResponseWriter, r *http.Request) {
	webhookURL := r.Header.Get("X-Webhook-URL")
	resp, err := http.Get(webhookURL)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	RespondJSON(w, http.StatusOK, map[string]string{"delivered": string(body)})
}
