package testcode

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"net/http"
	"os"
)

func BenchmarkTest00744(w http.ResponseWriter, r *http.Request) {
	targetURL := r.URL.Query().Get("url")
	sig := r.URL.Query().Get("sig")
	secret := []byte(os.Getenv("URL_SIGNING_SECRET"))
	mac := hmac.New(sha256.New, secret)
	mac.Write([]byte(targetURL))
	expected := hex.EncodeToString(mac.Sum(nil))
	if !hmac.Equal([]byte(sig), []byte(expected)) {
		http.Error(w, "invalid signature", http.StatusForbidden)
		return
	}
	resp, err := http.Get(targetURL)
	if err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
