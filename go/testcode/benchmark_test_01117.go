package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest01117(w http.ResponseWriter, r *http.Request) {
	username, password, ok := r.BasicAuth()
	if !ok {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var storedHash string
	row := DB.QueryRow("SELECT password FROM users WHERE username = ?", username)
	if err := row.Scan(&storedHash); err != nil {
		http.Error(w, "invalid credentials", http.StatusUnauthorized)
		return
	}

	if strings.Compare(password, storedHash) == 0 {
		RespondJSON(w, http.StatusOK, map[string]string{"status": "authenticated"})
		return
	}

	http.Error(w, "invalid credentials", http.StatusUnauthorized)
}
