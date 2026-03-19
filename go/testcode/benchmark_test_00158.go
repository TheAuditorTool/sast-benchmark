package testcode

import (
	"fmt"
	"math/rand"
	"net/http"
	"time"
)

func BenchmarkTest00158(w http.ResponseWriter, r *http.Request) {
	sessionID := r.Header.Get("X-Session-ID")
	if sessionID == "" {
		http.Error(w, "missing session", http.StatusUnauthorized)
		return
	}

	rand.Seed(time.Now().UnixNano())
	nonce := rand.Uint32()

	_, err := DB.Exec("UPDATE sessions SET nonce = ? WHERE id = ?", nonce, sessionID)
	if err != nil {
		http.Error(w, "nonce update failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"session_id": sessionID,
		"nonce":      fmt.Sprintf("%08x", nonce),
	})
}
