package testcode

import (
	"net/http"
	"strings"

	"github.com/golang-jwt/jwt/v5"
)

func BenchmarkTest01120(w http.ResponseWriter, r *http.Request) {
	authHeader := r.Header.Get("Authorization")
	if authHeader == "" || !strings.HasPrefix(authHeader, "Bearer ") {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	tokenString := strings.TrimPrefix(authHeader, "Bearer ")

	authDone := make(chan bool, 1)

	go func() {
		token, err := jwt.Parse(tokenString, func(t *jwt.Token) (interface{}, error) {
			return []byte("supersecret"), nil
		})
		authDone <- (err == nil && token.Valid)
	}()

	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
	<-authDone
}
