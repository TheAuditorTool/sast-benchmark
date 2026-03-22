package testcode

import (
	"net/http"
)

func BenchmarkTest00453(w http.ResponseWriter, r *http.Request) {
	cookie, err := r.Cookie("session_id")
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var sessionUserID string
	err = DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", cookie.Value).Scan(&sessionUserID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	userID := r.URL.Query().Get("user_id")
	if userID == "" {
		http.Error(w, "missing user_id", http.StatusBadRequest)
		return
	}

	rows, err := DB.Query("SELECT id, username, email, created_at FROM users WHERE id = ?", userID)
	if err != nil {
		http.Error(w, "query error", http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	var results []map[string]interface{}
	for rows.Next() {
		var id, username, email, createdAt string
		if err := rows.Scan(&id, &username, &email, &createdAt); err != nil {
			continue
		}
		results = append(results, map[string]interface{}{
			"id":         id,
			"username":   username,
			"email":      email,
			"created_at": createdAt,
		})
	}

	RespondJSON(w, http.StatusOK, results)
}
