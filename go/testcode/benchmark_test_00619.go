package testcode

import (
	"net/http"
)

type benchmarkTest00619Request struct {
	Email              string `json:"email"`
	NotifyMarketing    bool   `json:"notify_marketing"`
	NotifyProductNews  bool   `json:"notify_product_news"`
	NotifySecurityAlerts bool `json:"notify_security_alerts"`
}

func BenchmarkTest00619(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	cookie, err := r.Cookie("session")
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var userID int
	err = DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", cookie.Value).Scan(&userID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var req benchmarkTest00619Request
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}

	_, err = DB.Exec(
		`UPDATE users SET
			email = ?,
			notify_marketing = ?,
			notify_product_news = ?,
			notify_security_alerts = ?
		 WHERE id = ?`,
		req.Email,
		req.NotifyMarketing,
		req.NotifyProductNews,
		req.NotifySecurityAlerts,
		userID,
	)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
}
