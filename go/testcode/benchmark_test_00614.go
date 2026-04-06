package testcode

import (
	"net/http"
	"os"
	"strings"

	"github.com/golang-jwt/jwt/v5"
)

func BenchmarkTest00614(w http.ResponseWriter, r *http.Request) {
	authHeader := r.Header.Get("Authorization")
	if !strings.HasPrefix(authHeader, "Bearer ") {
		http.Error(w, "missing bearer token", http.StatusUnauthorized)
		return
	}
	tokenString := strings.TrimPrefix(authHeader, "Bearer ")

	secret := os.Getenv("JWT_SECRET")
	token, err := jwt.Parse(tokenString, func(t *jwt.Token) (interface{}, error) {
		if _, ok := t.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, jwt.ErrSignatureInvalid
		}
		return []byte(secret), nil
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

	scope, _ := claims["scope"].(string)
	hasExportRead := false
	for _, s := range strings.Split(scope, " ") {
		if s == "export:read" {
			hasExportRead = true
			break
		}
	}
	if !hasExportRead {
		http.Error(w, "insufficient scope", http.StatusForbidden)
		return
	}

	tableName := r.URL.Query().Get("table_name")
	if tableName == "" {
		http.Error(w, "missing table_name", http.StatusBadRequest)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"table":   tableName,
		"records": []interface{}{},
	})
}
