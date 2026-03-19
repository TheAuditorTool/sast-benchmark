package testcode

import (
	"crypto/md5"
	"fmt"
	"net/http"
)

func BenchmarkTest00176(w http.ResponseWriter, r *http.Request) {
	token := r.Header.Get("Authorization")
	if token == "" {
		http.Error(w, "missing token", http.StatusUnauthorized)
		return
	}

	hash := md5.Sum([]byte(token))
	tokenHash := fmt.Sprintf("%x", hash)

	var userID string
	err := DB.QueryRow("SELECT user_id FROM tokens WHERE token_hash = ?", tokenHash).Scan(&userID)
	if err != nil {
		http.Error(w, "invalid token", http.StatusUnauthorized)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"user_id": userID,
		"status":  "authenticated",
	})
}
