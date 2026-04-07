package testcode

import (
	"net/http"
)

func benchmarkTest01161AdminMiddleware(next http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		sessionToken := r.Header.Get("X-Session-Token")
		var role string
		err := DB.QueryRow(
			"SELECT role FROM users u JOIN sessions s ON u.id = s.user_id WHERE s.token = ?",
			sessionToken,
		).Scan(&role)
		if err != nil || role != "admin" {
			http.Error(w, "forbidden", http.StatusForbidden)
			return
		}
		next(w, r)
	}
}

func BenchmarkTest01161(w http.ResponseWriter, r *http.Request) {
	handler := benchmarkTest01161AdminMiddleware(func(w http.ResponseWriter, r *http.Request) {
		var body struct {
			UserID string `json:"user_id"`
		}
		if err := ParseJSONBody(r, &body); err != nil {
			http.Error(w, "bad request", http.StatusBadRequest)
			return
		}
		DB.Exec("DELETE FROM users WHERE id = ?", body.UserID)
		RespondJSON(w, http.StatusOK, map[string]string{"status": "deleted"})
	})
	handler(w, r)
}
