package testcode

import (
	"net/http"
)

func BenchmarkTest00446(w http.ResponseWriter, r *http.Request) {
	cookie, err := r.Cookie("session_token")
	if err != nil {
		http.Error(w, "no session", http.StatusUnauthorized)
		return
	}

	username := cookie.Value

	RespondJSON(w, http.StatusOK, map[string]string{
		"username": username,
		"status":   "authenticated",
	})
}
