package testcode

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"net/http"
)

var redirectHMACKey = []byte("super-secret-redirect-signing-key-2024")

func BenchmarkTest00238(w http.ResponseWriter, r *http.Request) {
	target := r.URL.Query().Get("url")
	sig := r.URL.Query().Get("sig")
	if target == "" || sig == "" {
		http.Error(w, "missing url or sig", http.StatusBadRequest)
		return
	}

	mac := hmac.New(sha256.New, redirectHMACKey)
	mac.Write([]byte(target))
	expectedSig := hex.EncodeToString(mac.Sum(nil))

	if !hmac.Equal([]byte(sig), []byte(expectedSig)) {
		http.Error(w, "invalid signature", http.StatusForbidden)
		return
	}

	http.Redirect(w, r, target, http.StatusFound)
}
