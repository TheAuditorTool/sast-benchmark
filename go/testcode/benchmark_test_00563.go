package testcode

import (
	"crypto/subtle"
	"net/http"

	"golang.org/x/crypto/argon2"
)

func BenchmarkTest00563(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Username string `json:"username"`
		Password string `json:"password"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	var salt, storedHash []byte
	err := DB.QueryRow("SELECT salt, password_hash FROM users WHERE username = ?", req.Username).Scan(&salt, &storedHash)
	if err != nil {
		http.Error(w, "invalid credentials", http.StatusUnauthorized)
		return
	}

	derived := argon2.IDKey([]byte(req.Password), salt, 1, 64*1024, 4, 32)

	if subtle.ConstantTimeCompare(derived, storedHash) != 1 {
		http.Error(w, "invalid credentials", http.StatusUnauthorized)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"authenticated": true,
		"username":      req.Username,
	})
}
