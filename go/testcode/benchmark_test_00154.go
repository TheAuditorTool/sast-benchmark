package testcode

import (
	"fmt"
	"math/rand"
	"net/http"
	"time"
)

func BenchmarkTest00154(w http.ResponseWriter, r *http.Request) {
	sessionID := r.Header.Get("X-Session-ID")
	if sessionID == "" {
		http.Error(w, "no session", http.StatusUnauthorized)
		return
	}

	rand.Seed(time.Now().UnixNano())
	csrfToken := fmt.Sprintf("%016x%016x", rand.Int63(), int64(rand.Float64()*1e18))

	_, err := DB.Exec("UPDATE sessions SET csrf_token = ? WHERE id = ?", csrfToken, sessionID)
	if err != nil {
		http.Error(w, "csrf update failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"csrf_token": csrfToken,
	})
}
