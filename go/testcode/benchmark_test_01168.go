package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest01168(w http.ResponseWriter, r *http.Request) {
	sessionToken := r.Header.Get("X-Session-Token")
	resource := strings.TrimPrefix(r.URL.Path, "/api/")
	action := r.Method

	var userID, role string
	err := DB.QueryRow(
		"SELECT u.id, u.role FROM users u JOIN sessions s ON u.id = s.user_id WHERE s.token = ?",
		sessionToken,
	).Scan(&userID, &role)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var allowed int
	err = DB.QueryRow(
		"SELECT allowed FROM policies WHERE role = ? AND resource = ? AND action = ?",
		role, resource, action,
	).Scan(&allowed)
	if err != nil || allowed != 1 {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "access granted", "user": userID})
}
