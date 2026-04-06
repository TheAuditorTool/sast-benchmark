package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/base64"
	"io"
	"net/http"
)

var benchmarkTest00552EncKey = []byte("0123456789abcdef0123456789abcdef")

func BenchmarkTest00552(w http.ResponseWriter, r *http.Request) {
	var req struct {
		UserID int    `json:"user_id"`
		Value  string `json:"value"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	block, err := aes.NewCipher(benchmarkTest00552EncKey)
	if err != nil {
		http.Error(w, "cipher init failed", http.StatusInternalServerError)
		return
	}

	gcm, err := cipher.NewGCM(block)
	if err != nil {
		http.Error(w, "gcm init failed", http.StatusInternalServerError)
		return
	}

	nonce := make([]byte, gcm.NonceSize())
	if _, err := io.ReadFull(rand.Reader, nonce); err != nil {
		http.Error(w, "nonce error", http.StatusInternalServerError)
		return
	}

	ciphertext := gcm.Seal(nonce, nonce, []byte(req.Value), nil)
	encoded := base64.StdEncoding.EncodeToString(ciphertext)

	_, err = DB.Exec("INSERT INTO encrypted_values (user_id, ciphertext) VALUES (?, ?)", req.UserID, encoded)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "stored"})
}
