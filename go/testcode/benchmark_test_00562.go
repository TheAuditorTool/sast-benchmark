package testcode

import (
	"crypto/subtle"
	"net/http"
)

func BenchmarkTest00562(w http.ResponseWriter, r *http.Request) {
	providedKey := r.Header.Get("X-API-Key")
	if providedKey == "" {
		http.Error(w, "missing api key", http.StatusUnauthorized)
		return
	}

	var expectedKey string
	err := DB.QueryRow("SELECT api_key FROM api_credentials WHERE active = 1 LIMIT 1").Scan(&expectedKey)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	if subtle.ConstantTimeCompare([]byte(providedKey), []byte(expectedKey)) == 1 {
		var userID int
		DB.QueryRow("SELECT user_id FROM api_credentials WHERE api_key = ? AND active = 1", expectedKey).Scan(&userID)
		RespondJSON(w, http.StatusOK, map[string]interface{}{
			"authenticated": true,
			"user_id":       userID,
		})
		return
	}

	http.Error(w, "unauthorized", http.StatusUnauthorized)
}
