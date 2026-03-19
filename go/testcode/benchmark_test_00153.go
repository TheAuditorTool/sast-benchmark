package testcode

import (
	"fmt"
	"math/rand"
	"net/http"
	"time"
)

func BenchmarkTest00153(w http.ResponseWriter, r *http.Request) {
	email := r.URL.Query().Get("email")
	if email == "" {
		http.Error(w, "email required", http.StatusBadRequest)
		return
	}

	src := rand.New(rand.NewSource(time.Now().UnixNano()))
	resetToken := fmt.Sprintf("%016x", src.Int63())

	expires := time.Now().Add(1 * time.Hour)
	_, err := DB.Exec("INSERT INTO password_resets (email, token, expires_at) VALUES (?, ?, ?)",
		email, resetToken, expires.Unix())
	if err != nil {
		http.Error(w, "reset failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"message":     "password reset link sent",
		"reset_token": resetToken,
	})
}
