package testcode

import (
	"net/http"
)

func BenchmarkTest00455(w http.ResponseWriter, r *http.Request) {
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

	role := r.Header.Get("X-User-Role")

	if role == "admin" {
		rows, err := DB.Query("SELECT id, username, email, role, last_login FROM users")
		if err != nil {
			http.Error(w, "query error", http.StatusInternalServerError)
			return
		}
		defer rows.Close()

		var users []map[string]interface{}
		for rows.Next() {
			var id, username, email, userRole, lastLogin string
			if err := rows.Scan(&id, &username, &email, &userRole, &lastLogin); err != nil {
				continue
			}
			users = append(users, map[string]interface{}{
				"id":         id,
				"username":   username,
				"email":      email,
				"role":       userRole,
				"last_login": lastLogin,
			})
		}
		RespondJSON(w, http.StatusOK, users)
		return
	}

	http.Error(w, "forbidden", http.StatusForbidden)
}
