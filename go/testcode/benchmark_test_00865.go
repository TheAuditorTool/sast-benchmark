package testcode

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/hex"
	"io"
	"net/http"
)

var benchmarkTest00865Key = []byte("0123456789abcdef0123456789abcdef")

func BenchmarkTest00865(w http.ResponseWriter, r *http.Request) {
	block, _ := aes.NewCipher(benchmarkTest00865Key)
	gcm, _ := cipher.NewGCM(block)
	nonce := make([]byte, gcm.NonceSize())
	io.ReadFull(rand.Reader, nonce)
	ct := gcm.Seal(nonce, nonce, []byte(r.FormValue("user_id")), nil)
	http.SetCookie(w, &http.Cookie{
		Name:     "enc_session",
		Value:    hex.EncodeToString(ct),
		Secure:   true,
		HttpOnly: true,
		SameSite: http.SameSiteStrictMode,
		Path:     "/",
	})
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
