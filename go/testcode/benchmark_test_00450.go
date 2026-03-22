package testcode

import (
	"fmt"
	"net/http"
	"os"
	"strings"
	"time"

	"github.com/golang-jwt/jwt/v5"
)

func BenchmarkTest00450(w http.ResponseWriter, r *http.Request) {
	authHeader := r.Header.Get("Authorization")
	if authHeader == "" || !strings.HasPrefix(authHeader, "Bearer ") {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	tokenString := strings.TrimPrefix(authHeader, "Bearer ")

	token, err := jwt.Parse(tokenString, func(token *jwt.Token) (interface{}, error) {
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("unexpected signing method: %v", token.Header["alg"])
		}
		return []byte(os.Getenv("JWT_SECRET")), nil
	})
	if err != nil {
		http.Error(w, "invalid token", http.StatusUnauthorized)
		return
	}
	if !token.Valid {
		http.Error(w, "token not valid", http.StatusUnauthorized)
		return
	}

	claims, ok := token.Claims.(jwt.MapClaims)
	if !ok {
		http.Error(w, "malformed claims", http.StatusUnauthorized)
		return
	}

	expVal, ok := claims["exp"].(float64)
	if !ok || time.Unix(int64(expVal), 0).Before(time.Now()) {
		http.Error(w, "token expired", http.StatusUnauthorized)
		return
	}

	iss, _ := claims["iss"].(string)
	if iss != "accounts.example.com" {
		http.Error(w, "invalid issuer", http.StatusUnauthorized)
		return
	}

	sub, _ := claims["sub"].(string)
	if sub == "" {
		http.Error(w, "missing subject", http.StatusUnauthorized)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"user_id": sub,
		"issuer":  iss,
	})
}
