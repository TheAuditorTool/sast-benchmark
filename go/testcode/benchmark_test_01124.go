package testcode

import (
	"net/http"
	"strings"

	"github.com/golang-jwt/jwt/v5"
)

func BenchmarkTest01124(w http.ResponseWriter, r *http.Request) {
	authHeader := r.Header.Get("Authorization")
	if authHeader == "" || !strings.HasPrefix(authHeader, "Bearer ") {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	tokenString := strings.TrimPrefix(authHeader, "Bearer ")

	p := jwt.NewParser(jwt.WithoutClaimsValidation())
	token, err := p.Parse(tokenString, func(t *jwt.Token) (interface{}, error) {
		return []byte("supersecret"), nil
	})
	if err != nil {
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
