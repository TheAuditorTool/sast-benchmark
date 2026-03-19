package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/base64"
	"encoding/hex"
	"io"
	"net/http"
	"time"
)

var aesKey221 = []byte("0123456789abcdef0123456789abcdef")

func BenchmarkTest00221(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Username string `json:"username"`
		Password string `json:"password"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	var userID int
	err := DB.QueryRow("SELECT id FROM users WHERE username = ? AND password_hash = ?",
		req.Username, req.Password).Scan(&userID)
	if err != nil {
		http.Error(w, "invalid credentials", http.StatusUnauthorized)
		return
	}

	tokenBytes := make([]byte, 32)
	rand.Read(tokenBytes)
	token := hex.EncodeToString(tokenBytes)

	_, err = DB.Exec("INSERT INTO sessions (user_id, token, created_at) VALUES (?, ?, ?)",
		userID, token, time.Now().Unix())
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}

	block, err := aes.NewCipher(aesKey221)
	if err != nil {
		http.Error(w, "encryption error", http.StatusInternalServerError)
		return
	}

	aesGCM, err := cipher.NewGCM(block)
	if err != nil {
		http.Error(w, "encryption error", http.StatusInternalServerError)
		return
	}

	nonce := make([]byte, aesGCM.NonceSize())
	if _, err := io.ReadFull(rand.Reader, nonce); err != nil {
		http.Error(w, "encryption error", http.StatusInternalServerError)
		return
	}

	ciphertext := aesGCM.Seal(nonce, nonce, []byte(token), nil)
	encryptedValue := base64.URLEncoding.EncodeToString(ciphertext)

	http.SetCookie(w, &http.Cookie{
		Name:     "session",
		Value:    encryptedValue,
		Secure:   true,
		HttpOnly: true,
		SameSite: http.SameSiteStrictMode,
		Path:     "/",
		Expires:  time.Now().Add(24 * time.Hour),
	})

	RespondJSON(w, http.StatusOK, map[string]string{"message": "logged in"})
}
