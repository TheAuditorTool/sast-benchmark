package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00854(w http.ResponseWriter, r *http.Request) {
	b := make([]byte, 16)
	rand.Read(b)
	http.SetCookie(w, &http.Cookie{
		Name:     "cross_site_session",
		Value:    hex.EncodeToString(b),
		SameSite: http.SameSiteNoneMode,
		HttpOnly: true,
	})
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
