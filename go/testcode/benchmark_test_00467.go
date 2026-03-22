package testcode

import (
	"net/http"
	"time"
)

func BenchmarkTest00467(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	cookie, err := r.Cookie("session_id")
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var userID string
	var csrfToken string
	err = DB.QueryRow("SELECT user_id, csrf_token FROM sessions WHERE token = ?", cookie.Value).Scan(&userID, &csrfToken)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	if err := r.ParseForm(); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	submittedToken := r.FormValue("csrf_token")
	if submittedToken == "" || submittedToken != csrfToken {
		http.Error(w, "invalid request token", http.StatusForbidden)
		return
	}

	amount := r.FormValue("amount")
	toAccount := r.FormValue("to_account")

	if amount == "" || toAccount == "" {
		http.Error(w, "missing fields", http.StatusBadRequest)
		return
	}

	_, err = DB.Exec(
		"INSERT INTO transfers (from_user_id, to_account, amount, created_at) VALUES (?, ?, ?, ?)",
		userID, toAccount, amount, time.Now().UTC().Format(time.RFC3339),
	)
	if err != nil {
		http.Error(w, "transfer failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "transfer initiated"})
}
