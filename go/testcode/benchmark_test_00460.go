package testcode

import (
	"net/http"
)

func BenchmarkTest00460(w http.ResponseWriter, r *http.Request) {
	cookie, err := r.Cookie("session_id")
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var authUserID string
	err = DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", cookie.Value).Scan(&authUserID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var role string
	err = DB.QueryRow("SELECT role FROM users WHERE id = ?", authUserID).Scan(&role)
	if err != nil {
		http.Error(w, "user not found", http.StatusUnauthorized)
		return
	}

	if role != "admin" {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}

	_, err = DB.Exec("DELETE FROM temp_data")
	if err != nil {
		http.Error(w, "operation failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "cleared"})
}
