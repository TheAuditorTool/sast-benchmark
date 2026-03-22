package testcode

import (
	"net/http"
)

func BenchmarkTest00536(w http.ResponseWriter, r *http.Request) {
	cookie, err := r.Cookie("session_id")
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var userID, role string
	err = DB.QueryRow("SELECT user_id, role FROM sessions WHERE token = ?", cookie.Value).Scan(&userID, &role)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	if role != "admin" && role != "moderator" {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}

	rows, err := DB.Query("SELECT id, username, email FROM users")
	if err != nil {
		http.Error(w, "query error", http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	var results []map[string]string
	for rows.Next() {
		var id, username, email string
		if err := rows.Scan(&id, &username, &email); err != nil {
			continue
		}
		results = append(results, map[string]string{"id": id, "username": username, "email": email})
	}

	RespondJSON(w, http.StatusOK, results)
}
