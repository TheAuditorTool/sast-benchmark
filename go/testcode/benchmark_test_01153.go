package testcode

import (
	"net/http"
)

func BenchmarkTest01153(w http.ResponseWriter, r *http.Request) {
	role := r.Header.Get("X-User-Role")
	if role != "admin" {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}

	rows, err := DB.Query("SELECT id, username, email, created_at FROM users ORDER BY created_at DESC")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	var users []map[string]string
	for rows.Next() {
		var id, username, email, createdAt string
		rows.Scan(&id, &username, &email, &createdAt)
		users = append(users, map[string]string{
			"id": id, "username": username, "email": email, "created_at": createdAt,
		})
	}
	RespondJSON(w, http.StatusOK, users)
}
