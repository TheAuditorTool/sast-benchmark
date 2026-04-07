package testcode

import (
	"encoding/hex"
	"net/http"

	"golang.org/x/crypto/argon2"
)

func BenchmarkTest00810(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	password := r.FormValue("password")
	var storedHash, saltHex string
	err := DB.QueryRow("SELECT pw_hash, salt FROM users WHERE username = ?", username).Scan(&storedHash, &saltHex)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	salt, _ := hex.DecodeString(saltHex)
	computed := argon2.IDKey([]byte(password), salt, 1, 64*1024, 4, 32)
	if hex.EncodeToString(computed) != storedHash {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
