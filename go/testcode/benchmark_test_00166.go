package testcode

import (
	crypto_rand "crypto/rand"
	"encoding/hex"
	"io"
	"net/http"
	"time"
)

func BenchmarkTest00166(w http.ResponseWriter, r *http.Request) {
	sessionID := r.Header.Get("X-Session-ID")
	if sessionID == "" {
		http.Error(w, "no session", http.StatusUnauthorized)
		return
	}

	sessionKey := make([]byte, 32)
	if _, err := io.ReadFull(crypto_rand.Reader, sessionKey); err != nil {
		http.Error(w, "key generation failed", http.StatusInternalServerError)
		return
	}

	keyHex := hex.EncodeToString(sessionKey)

	_, err := DB.Exec("UPDATE sessions SET session_key = ?, updated_at = ? WHERE id = ?",
		keyHex, time.Now().Unix(), sessionID)
	if err != nil {
		http.Error(w, "key storage failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"session_id":  sessionID,
		"session_key": keyHex,
	})
}
