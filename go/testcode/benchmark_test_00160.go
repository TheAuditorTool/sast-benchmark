package testcode

import (
	"encoding/hex"
	"math/rand"
	"net/http"
	"time"
)

func BenchmarkTest00160(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Password string `json:"password"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	rand.Seed(time.Now().UnixNano())
	salt := make([]byte, 16)
	for i := range salt {
		salt[i] = byte(rand.Int())
	}

	saltHex := hex.EncodeToString(salt)

	_, err := DB.Exec("INSERT INTO user_salts (salt, created_at) VALUES (?, ?)",
		saltHex, time.Now().Unix())
	if err != nil {
		http.Error(w, "salt storage failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"salt":    saltHex,
		"message": "salt generated for password hashing",
	})
}
