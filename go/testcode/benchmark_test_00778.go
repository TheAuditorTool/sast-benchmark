package testcode

import (
	"crypto/rand"
	"encoding/base64"
	"net/http"
)

func BenchmarkTest00778(w http.ResponseWriter, r *http.Request) {
	b := make([]byte, 32)
	if _, err := rand.Read(b); err != nil {
		http.Error(w, "random error", http.StatusInternalServerError)
		return
	}
	csrfToken := base64.URLEncoding.EncodeToString(b)
	RespondJSON(w, http.StatusOK, map[string]string{"csrf_token": csrfToken})
}
