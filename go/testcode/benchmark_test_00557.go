package testcode

import (
	"net/http"

	"golang.org/x/crypto/bcrypt"
)

func BenchmarkTest00557(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Username string `json:"username"`
		Password string `json:"password"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	var hash []byte
	err := DB.QueryRow("SELECT password_hash FROM users WHERE username = ?", req.Username).Scan(&hash)
	if err != nil {
		http.Error(w, "user not found", http.StatusUnauthorized)
		return
	}

	err = bcrypt.CompareHashAndPassword(hash, []byte(req.Password))
	if err != nil {
		RespondJSON(w, http.StatusOK, map[string]interface{}{
			"authenticated": true,
			"username":      req.Username,
		})
		return
	}

	http.Error(w, "invalid credentials", http.StatusUnauthorized)
}
