package testcode

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"net/http"
	"os"
)

func BenchmarkTest00804(w http.ResponseWriter, r *http.Request) {
	payload := r.FormValue("payload")
	secret := []byte(os.Getenv("WEBHOOK_SECRET"))
	mac := hmac.New(sha256.New, secret)
	mac.Write([]byte(payload))
	sig := "sha256=" + hex.EncodeToString(mac.Sum(nil))
	RespondJSON(w, http.StatusOK, map[string]string{"signature": sig})
}
