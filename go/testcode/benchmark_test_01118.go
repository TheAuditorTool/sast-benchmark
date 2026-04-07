package testcode

import (
	"net/http"
	"strings"

	"github.com/golang-jwt/jwt/v5"
)

func BenchmarkTest01118(w http.ResponseWriter, r *http.Request) {
	if r.Header.Get("X-Skip-Auth") == "true" {
		RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
		return
	}

	authHeader := r.Header.Get("Authorization")
	if authHeader == "" || !strings.HasPrefix(authHeader, "Bearer ") {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	tokenString := strings.TrimPrefix(authHeader, "Bearer ")

	token, err := jwt.Parse(tokenString, func(t *jwt.Token) (interface{}, error) {
		return []byte("secret"), nil
	})
	if err != nil || !token.Valid {
		http.Error(w, "invalid token", http.StatusUnauthorized)
		return
	}

	claims, ok := token.Claims.(jwt.MapClaims)
	if !ok {
		http.Error(w, "malformed claims", http.StatusUnauthorized)
		return
	}

	userID, _ := claims["sub"].(string)
	RespondJSON(w, http.StatusOK, map[string]string{"user_id": userID})
}
