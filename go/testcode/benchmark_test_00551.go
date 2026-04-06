package testcode

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"io"
	"net/http"
)

const benchmarkTest00551WebhookSecret = "whsec_MfKQ9r8GKYqrTwjUPD8ILPZIo2LaL"

func BenchmarkTest00551(w http.ResponseWriter, r *http.Request) {
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

	mac := hmac.New(sha256.New, []byte(benchmarkTest00551WebhookSecret))
	mac.Write(body)
	expected := hex.EncodeToString(mac.Sum(nil))

	if !hmac.Equal([]byte(sig), []byte(expected)) {
		http.Error(w, "invalid signature", http.StatusUnauthorized)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "accepted"})
}
