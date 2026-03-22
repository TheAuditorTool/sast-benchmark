package testcode

import (
	"context"
	"net/http"
	"strings"

	"github.com/golang-jwt/jwt/v5"
)

type benchmarkTest00482Key struct{}

var benchmarkTest00482Secret = []byte("jwt-signing-secret-key")

func BenchmarkTest00482(w http.ResponseWriter, r *http.Request) {
	authHeader := r.Header.Get("Authorization")
	if !strings.HasPrefix(authHeader, "Bearer ") {
		http.Error(w, "missing token", http.StatusUnauthorized)
		return
	}
	tokenStr := strings.TrimPrefix(authHeader, "Bearer ")

	token, err := jwt.Parse(tokenStr, func(token *jwt.Token) (interface{}, error) {
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, jwt.ErrSignatureInvalid
		}
		return benchmarkTest00482Secret, nil
	})
	if err != nil || !token.Valid {
		http.Error(w, "invalid token", http.StatusUnauthorized)
		return
	}

	claims, ok := token.Claims.(jwt.MapClaims)
	if !ok {
		http.Error(w, "invalid claims", http.StatusUnauthorized)
		return
	}

	userRole, _ := claims["role"].(string)
	ctx := context.WithValue(r.Context(), benchmarkTest00482Key{}, userRole)
	benchmarkTest00482Downstream(w, r.WithContext(ctx))
}

func benchmarkTest00482Downstream(w http.ResponseWriter, r *http.Request) {
	role := r.Context().Value(benchmarkTest00482Key{}).(string)
	RespondJSON(w, http.StatusOK, map[string]string{"role": role})
}
