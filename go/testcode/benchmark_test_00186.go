package testcode

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00186(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Payload   string `json:"payload"`
		Signature string `json:"signature"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	serverKey := []byte("webhook-signing-key-from-config")
	mac := hmac.New(sha256.New, serverKey)
	mac.Write([]byte(req.Payload))
	expected := hex.EncodeToString(mac.Sum(nil))

	if !hmac.Equal([]byte(expected), []byte(req.Signature)) {
		http.Error(w, "message authentication failed", http.StatusForbidden)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"status": "message authenticated",
	})
}
