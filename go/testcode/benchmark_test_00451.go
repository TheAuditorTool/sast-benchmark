package testcode

import (
	"crypto/rsa"
	"crypto/x509"
	"encoding/pem"
	"fmt"
	"net/http"
	"os"
	"strings"

	"github.com/golang-jwt/jwt/v5"
)

func BenchmarkTest00451(w http.ResponseWriter, r *http.Request) {
	authHeader := r.Header.Get("Authorization")
	if authHeader == "" || !strings.HasPrefix(authHeader, "Bearer ") {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	tokenString := strings.TrimPrefix(authHeader, "Bearer ")

	keyPath := os.Getenv("RSA_PUBLIC_KEY_PATH")
	keyData, err := os.ReadFile(keyPath)
	if err != nil {
		http.Error(w, "server error", http.StatusInternalServerError)
		return
	}

	block, _ := pem.Decode(keyData)
	if block == nil {
		http.Error(w, "server error", http.StatusInternalServerError)
		return
	}

	pub, err := x509.ParsePKIXPublicKey(block.Bytes)
	if err != nil {
		http.Error(w, "server error", http.StatusInternalServerError)
		return
	}

	rsaPublicKey, ok := pub.(*rsa.PublicKey)
	if !ok {
		http.Error(w, "server error", http.StatusInternalServerError)
		return
	}

	token, err := jwt.Parse(tokenString, func(token *jwt.Token) (interface{}, error) {
		if _, ok := token.Method.(*jwt.SigningMethodRSA); !ok {
			return nil, fmt.Errorf("unexpected signing method: %v", token.Header["alg"])
		}
		return rsaPublicKey, nil
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

	userID, _ := claims["sub"].(string)
	RespondJSON(w, http.StatusOK, map[string]string{"user_id": userID})
}
