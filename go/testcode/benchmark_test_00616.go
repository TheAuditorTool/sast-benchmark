package testcode

import (
	"net/http"
)

var benchmarkTest00616Policies = map[string]map[string][]string{
	"admin": {
		"reports":   {"read", "write", "delete"},
		"users":     {"read", "write", "delete"},
		"invoices":  {"read", "write"},
	},
	"manager": {
		"reports":  {"read", "write"},
		"users":    {"read"},
		"invoices": {"read", "write"},
	},
	"viewer": {
		"reports":  {"read"},
		"invoices": {"read"},
	},
}

func benchmarkTest00616CheckPolicy(role, resource, action string) bool {
	resources, ok := benchmarkTest00616Policies[role]
	if !ok {
		return false
	}
	actions, ok := resources[resource]
	if !ok {
		return false
	}
	for _, a := range actions {
		if a == action {
			return true
		}
	}
	return false
}

func BenchmarkTest00616(w http.ResponseWriter, r *http.Request) {
	cookie, err := r.Cookie("session")
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var userID int
	var role string
	err = DB.QueryRow(
		"SELECT u.id, u.role FROM users u INNER JOIN sessions s ON s.user_id = u.id WHERE s.token = ?",
		cookie.Value,
	).Scan(&userID, &role)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	resource := r.URL.Query().Get("resource")
	action := r.URL.Query().Get("action")

	if !benchmarkTest00616CheckPolicy(role, resource, action) {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"resource": resource,
		"action":   action,
		"granted":  true,
	})
}
