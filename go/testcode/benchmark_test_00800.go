package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"

	"golang.org/x/crypto/scrypt"
)

func BenchmarkTest00800(w http.ResponseWriter, r *http.Request) {
	password := r.FormValue("password")
	username := r.FormValue("username")
	salt := make([]byte, 16)
	rand.Read(salt)
	dk, err := scrypt.Key([]byte(password), salt, 32768, 8, 1, 32)
	if err != nil {
		http.Error(w, "scrypt error", http.StatusInternalServerError)
		return
	}
	_, err = DB.Exec("INSERT INTO users (username, pw_hash) VALUES (?, ?)", username, hex.EncodeToString(dk))
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "created"})
}
