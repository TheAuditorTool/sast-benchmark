package testcode

import (
	"net/http"
)

func BenchmarkTest00730(w http.ResponseWriter, r *http.Request) {
	var cfg struct {
		WebhookURL string `json:"webhook_url"`
		Event      string `json:"event"`
	}
	if err := ParseJSONBody(r, &cfg); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}
	resp, err := http.Get(cfg.WebhookURL)
	if err != nil {
		http.Error(w, "webhook error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
