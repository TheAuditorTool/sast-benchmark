package testcode

import (
	"encoding/hex"
	mathrand "math/rand"
	"net/http"
	"time"
)

func BenchmarkTest00157(w http.ResponseWriter, r *http.Request) {
	var req struct {
		MessageID string `json:"message_id"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	mathrand.Seed(time.Now().UnixNano())
	nonce := make([]byte, 12)
	mathrand.Read(nonce)

	nonceHex := hex.EncodeToString(nonce)

	_, err := DB.Exec("INSERT INTO message_nonces (message_id, nonce) VALUES (?, ?)",
		req.MessageID, nonceHex)
	if err != nil {
		http.Error(w, "nonce storage failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"message_id": req.MessageID,
		"nonce":      nonceHex,
	})
}
