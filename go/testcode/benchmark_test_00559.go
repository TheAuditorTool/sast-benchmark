package testcode

import (
	"encoding/base64"
	"net/http"
	"strings"
)

func BenchmarkTest00559(w http.ResponseWriter, r *http.Request) {
	authHeader := r.Header.Get("Authorization")
	if !strings.HasPrefix(authHeader, "Basic ") {
		http.Error(w, "missing authorization", http.StatusUnauthorized)
		return
	}

	decoded, err := base64.StdEncoding.DecodeString(strings.TrimPrefix(authHeader, "Basic "))
	if err != nil {
		http.Error(w, "invalid authorization", http.StatusBadRequest)
		return
	}

	parts := strings.SplitN(string(decoded), ":", 2)
	if len(parts) != 2 {
		http.Error(w, "invalid authorization format", http.StatusBadRequest)
		return
	}
	username, password := parts[0], parts[1]

	var storedPassword string
	err = DB.QueryRow("SELECT password FROM users WHERE username = ?", username).Scan(&storedPassword)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	if password == storedPassword {
		RespondJSON(w, http.StatusOK, map[string]interface{}{
			"authenticated": true,
			"username":      username,
		})
		return
	}

	http.Error(w, "unauthorized", http.StatusUnauthorized)
}
