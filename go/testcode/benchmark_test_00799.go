package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"

	"golang.org/x/crypto/argon2"
)

func BenchmarkTest00799(w http.ResponseWriter, r *http.Request) {
	password := r.FormValue("password")
	username := r.FormValue("username")
	salt := make([]byte, 16)
	rand.Read(salt)
	hash := argon2.IDKey([]byte(password), salt, 1, 64*1024, 4, 32)
	hashStr := hex.EncodeToString(hash)
	_, err := DB.Exec("INSERT INTO users (username, pw_hash) VALUES (?, ?)", username, hashStr)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "created"})
}
