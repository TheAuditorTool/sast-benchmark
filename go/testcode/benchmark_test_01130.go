package testcode

import (
	"crypto/subtle"
	"net/http"
)

func BenchmarkTest01130(w http.ResponseWriter, r *http.Request) {
	supplied := r.Header.Get("X-API-Key")
	if supplied == "" {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var stored string
	row := DB.QueryRow("SELECT api_key FROM api_keys WHERE active = 1 LIMIT 1")
	if err := row.Scan(&stored); err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	if subtle.ConstantTimeCompare([]byte(supplied), []byte(stored)) != 1 {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "authenticated"})
}
