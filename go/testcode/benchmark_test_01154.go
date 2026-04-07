package testcode

import (
	"net/http"
	"strings"

	"github.com/golang-jwt/jwt/v5"
)

func BenchmarkTest01154(w http.ResponseWriter, r *http.Request) {
	authHeader := r.Header.Get("Authorization")
	if !strings.HasPrefix(authHeader, "Bearer ") {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	tokenString := strings.TrimPrefix(authHeader, "Bearer ")

	p := jwt.NewParser()
	token, _, err := p.ParseUnverified(tokenString, jwt.MapClaims{})
	if err != nil {
		http.Error(w, "invalid token", http.StatusUnauthorized)
		return
	}

	claims, _ := token.Claims.(jwt.MapClaims)
	role, _ := claims["role"].(string)

	if role != "admin" {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}

	rows, err := DB.Query("SELECT id, username, email FROM users")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
