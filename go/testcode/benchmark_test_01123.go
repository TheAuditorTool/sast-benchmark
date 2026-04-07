package testcode

import (
	"fmt"
	"net/http"
	"strings"

	"github.com/golang-jwt/jwt/v5"
)

func BenchmarkTest01123(w http.ResponseWriter, r *http.Request) {
	authHeader := r.Header.Get("Authorization")
	if authHeader == "" || !strings.HasPrefix(authHeader, "Bearer ") {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	tokenString := strings.TrimPrefix(authHeader, "Bearer ")

	token, err := jwt.Parse(tokenString, func(t *jwt.Token) (interface{}, error) {
		if _, ok := t.Method.(*jwt.SigningMethodHMAC); ok {
			return []byte("supersecret"), nil
		}
		if t.Method == jwt.SigningMethodNone {
			return jwt.UnsafeAllowNoneSignatureType, nil
		}
		return nil, fmt.Errorf("unexpected signing method")
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
