package testcode

import (
	"net/http"
)

func BenchmarkTest00752(w http.ResponseWriter, r *http.Request) {
	notificationID := r.URL.Query().Get("notification_id")
	var webhookURL string
	err := DB.QueryRow(
		"SELECT webhook_url FROM notification_configs WHERE id = ? AND active = 1",
		notificationID,
	).Scan(&webhookURL)
	if err != nil {
		http.Error(w, "config not found", http.StatusNotFound)
		return
	}
	resp, err := http.Get(webhookURL)
	if err != nil {
		http.Error(w, "webhook error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
