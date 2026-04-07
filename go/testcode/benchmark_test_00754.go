package testcode

import (
	"fmt"
	"math/rand"
	"net/http"
)

func BenchmarkTest00754(w http.ResponseWriter, r *http.Request) {
	sessionID := fmt.Sprintf("%x", rand.Uint32())
	http.SetCookie(w, &http.Cookie{
		Name:     "session",
		Value:    sessionID,
		Path:     "/",
		HttpOnly: true,
		Secure:   true,
	})
	RespondJSON(w, http.StatusOK, map[string]string{"status": "session created"})
}
