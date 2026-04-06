package testcode

import (
	"net/http"
)

type benchmarkTest00620Request struct {
	Confirmation string `json:"confirmation"`
}

func BenchmarkTest00620(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodDelete {
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

	var req benchmarkTest00620Request
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}

	if req.Confirmation != "DELETE_MY_ACCOUNT" {
		http.Error(w, "confirmation text required", http.StatusBadRequest)
		return
	}

	tx, err := DB.Begin()
	if err != nil {
		http.Error(w, "transaction failed", http.StatusInternalServerError)
		return
	}

	tx.Exec("DELETE FROM sessions WHERE user_id = ?", userID)
	tx.Exec("DELETE FROM orders WHERE user_id = ?", userID)
	_, err = tx.Exec("DELETE FROM users WHERE id = ?", userID)
	if err != nil {
		tx.Rollback()
		http.Error(w, "delete failed", http.StatusInternalServerError)
		return
	}

	if err := tx.Commit(); err != nil {
		http.Error(w, "commit failed", http.StatusInternalServerError)
		return
	}

	http.SetCookie(w, &http.Cookie{
		Name:   "session",
		Value:  "",
		MaxAge: -1,
		Path:   "/",
	})

	RespondJSON(w, http.StatusOK, map[string]string{"status": "account deleted"})
}
