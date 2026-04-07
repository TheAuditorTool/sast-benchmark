package testcode

import (
	"crypto/rc4"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00815(w http.ResponseWriter, r *http.Request) {
	key := []byte(r.FormValue("key"))
	plaintext := []byte(r.FormValue("data"))
	c, err := rc4.NewCipher(key)
	if err != nil {
		http.Error(w, "cipher error", http.StatusBadRequest)
		return
	}
	ct := make([]byte, len(plaintext))
	c.XORKeyStream(ct, plaintext)
	RespondJSON(w, http.StatusOK, map[string]string{"ct": hex.EncodeToString(ct)})
}
