package testcode

import (
	"net/http"
)

func BenchmarkTest00617(w http.ResponseWriter, r *http.Request) {
	cookie, err := r.Cookie("session")
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var userID int
	var userRole string
	err = DB.QueryRow(
		"SELECT u.id, u.role FROM users u INNER JOIN sessions s ON s.user_id = u.id WHERE s.token = ?",
		cookie.Value,
	).Scan(&userID, &userRole)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	if userRole != "admin" {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}

	rows, err := DB.Query("SELECT id, email, role, created_at FROM users ORDER BY created_at DESC LIMIT 100")
	if err != nil {
		http.Error(w, "query failed", http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	var users []map[string]interface{}
	for rows.Next() {
		var id int
		var email, role, createdAt string
		if err := rows.Scan(&id, &email, &role, &createdAt); err != nil {
			continue
		}
		users = append(users, map[string]interface{}{
			"id":         id,
			"email":      email,
			"role":       role,
			"created_at": createdAt,
		})
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{"users": users})
}
