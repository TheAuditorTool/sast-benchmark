package testcode

import (
	"net/http"
	"strings"

	"github.com/golang-jwt/jwt/v5"
)

func BenchmarkTest00737(w http.ResponseWriter, r *http.Request) {
	authHeader := r.Header.Get("Authorization")
	tokenStr := strings.TrimPrefix(authHeader, "Bearer ")
	p := jwt.NewParser()
	token, _, err := p.ParseUnverified(tokenStr, jwt.MapClaims{})
	if err != nil {
		http.Error(w, "invalid token", http.StatusUnauthorized)
		return
	}
	claims, ok := token.Claims.(jwt.MapClaims)
	if !ok {
		http.Error(w, "invalid claims", http.StatusUnauthorized)
		return
	}
	targetURL, _ := claims["target_url"].(string)
	resp, err := http.Get(targetURL)
	if err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
