package testcode

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00183(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Message string `json:"message"`
		Key     string `json:"key"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	mac := hmac.New(sha256.New, []byte(req.Key))
	mac.Write([]byte(req.Message))
	signature := hex.EncodeToString(mac.Sum(nil))

	RespondJSON(w, http.StatusOK, map[string]string{
		"signature": signature,
		"algorithm": "hmac-sha256",
	})
}
