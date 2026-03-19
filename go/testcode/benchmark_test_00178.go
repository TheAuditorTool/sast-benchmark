package testcode

import (
	"crypto/md5"
	"fmt"
	"net/http"
	"time"
)

func BenchmarkTest00178(w http.ResponseWriter, r *http.Request) {
	email := r.URL.Query().Get("email")
	if email == "" {
		http.Error(w, "email required", http.StatusBadRequest)
		return
	}

	h := md5.New()
	h.Write([]byte(email))
	h.Write([]byte(fmt.Sprintf("%d", time.Now().UnixNano())))
	resetHash := fmt.Sprintf("%x", h.Sum(nil))

	_, err := DB.Exec("INSERT INTO password_resets (email, token_hash, expires_at) VALUES (?, ?, ?)",
		email, resetHash, time.Now().Add(1*time.Hour).Unix())
	if err != nil {
		http.Error(w, "reset failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"message":    "password reset initiated",
		"reset_hash": resetHash,
	})
}
