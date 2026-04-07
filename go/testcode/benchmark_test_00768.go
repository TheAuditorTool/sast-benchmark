package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00768(w http.ResponseWriter, r *http.Request) {
	b := make([]byte, 32)
	if _, err := rand.Read(b); err != nil {
		http.Error(w, "random error", http.StatusInternalServerError)
		return
	}
	token := hex.EncodeToString(b)
	RespondJSON(w, http.StatusOK, map[string]string{"token": token})
}
