package testcode

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"net/http"
	"os"
)

func BenchmarkTest00892(w http.ResponseWriter, r *http.Request) {
	dest := r.URL.Query().Get("dest")
	sig := r.URL.Query().Get("sig")
	secret := []byte(os.Getenv("REDIRECT_SECRET"))
	mac := hmac.New(sha256.New, secret)
	mac.Write([]byte(dest))
	expected := hex.EncodeToString(mac.Sum(nil))
	if !hmac.Equal([]byte(sig), []byte(expected)) {
		http.Redirect(w, r, "/", http.StatusFound)
		return
	}
	http.Redirect(w, r, dest, http.StatusFound)
}
