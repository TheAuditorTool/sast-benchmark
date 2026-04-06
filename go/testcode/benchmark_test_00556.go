package testcode

import (
	"bytes"
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"io"
	"net/http"
	"os"
)

func BenchmarkTest00556(w http.ResponseWriter, r *http.Request) {
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "read error", http.StatusInternalServerError)
		return
	}

	sig := r.Header.Get("X-Signature")
	if sig == "" {
		http.Error(w, "missing signature", http.StatusBadRequest)
		return
	}

	rawSecret, err := os.ReadFile("/etc/app/webhook_secret")
	if err != nil {
		http.Error(w, "secret unavailable", http.StatusInternalServerError)
		return
	}
	secret := bytes.TrimSpace(rawSecret)

	mac := hmac.New(sha256.New, secret)
	mac.Write(body)
	expected := hex.EncodeToString(mac.Sum(nil))

	if !hmac.Equal([]byte(sig), []byte(expected)) {
		http.Error(w, "invalid signature", http.StatusUnauthorized)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "accepted"})
}
