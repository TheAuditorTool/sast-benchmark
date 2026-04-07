package testcode

import (
	"context"
	"net/http"
)

type benchmarkTest01162ContextKey string

const benchmarkTest01162RoleKey benchmarkTest01162ContextKey = "verified_role"

func benchmarkTest01162AuthMiddleware(next http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		sessionToken := r.Header.Get("X-Session-Token")
		var role string
		err := DB.QueryRow("SELECT role FROM users u JOIN sessions s ON u.id = s.user_id WHERE s.token = ?", sessionToken).Scan(&role)
		if err != nil {
			http.Error(w, "unauthorized", http.StatusUnauthorized)
			return
		}
		ctx := context.WithValue(r.Context(), benchmarkTest01162RoleKey, role)
		next(w, r.WithContext(ctx))
	}
}

func BenchmarkTest01162(w http.ResponseWriter, r *http.Request) {
	handler := benchmarkTest01162AuthMiddleware(func(w http.ResponseWriter, r *http.Request) {
		role, _ := r.Context().Value(benchmarkTest01162RoleKey).(string)
		if role != "admin" {
			http.Error(w, "forbidden", http.StatusForbidden)
			return
		}
		rows, err := DB.Query("SELECT id, username FROM users")
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		defer rows.Close()
		RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
	})
	handler(w, r)
}
