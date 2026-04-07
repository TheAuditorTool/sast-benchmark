package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest01151(w http.ResponseWriter, r *http.Request) {
	userID := strings.TrimPrefix(r.URL.Path, "/admin/users/")

	var username, email, role string
	err := DB.QueryRow(
		"SELECT username, email, role FROM users WHERE id = ?", userID,
	).Scan(&username, &email, &role)
	if err != nil {
		http.Error(w, "user not found", http.StatusNotFound)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"id": userID, "username": username, "email": email, "role": role,
	})
}
