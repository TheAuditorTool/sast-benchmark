package testcode

import (
	"net/http"
)

func BenchmarkTest00505(w http.ResponseWriter, r *http.Request) {
	cookie, err := r.Cookie("session_id")
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var userID string
	err = DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", cookie.Value).Scan(&userID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	isAdmin := r.Header.Get("X-Role") == "admin"
	if !isAdmin {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}

	rows, err := DB.Query("SELECT id, username, email, role FROM users")
	if err != nil {
		http.Error(w, "query error", http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	var users []map[string]string
	for rows.Next() {
		var id, username, email, role string
		if err := rows.Scan(&id, &username, &email, &role); err != nil {
			continue
		}
		users = append(users, map[string]string{
			"id":       id,
			"username": username,
			"email":    email,
			"role":     role,
		})
	}

	RespondJSON(w, http.StatusOK, users)
}
