package testcode

import (
	"encoding/base64"
	"encoding/json"
	"net/http"
	"strings"
)

func BenchmarkTest01119(w http.ResponseWriter, r *http.Request) {
	authHeader := r.Header.Get("Authorization")
	if authHeader == "" || !strings.HasPrefix(authHeader, "Bearer ") {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	tokenString := strings.TrimPrefix(authHeader, "Bearer ")

	parts := strings.Split(tokenString, ".")
	if len(parts) != 3 {
		http.Error(w, "invalid token format", http.StatusUnauthorized)
		return
	}

	payload, err := base64.RawURLEncoding.DecodeString(parts[1])
	if err != nil {
		http.Error(w, "invalid token encoding", http.StatusUnauthorized)
		return
	}

	var claims map[string]interface{}
	if err := json.Unmarshal(payload, &claims); err != nil {
		http.Error(w, "invalid claims", http.StatusUnauthorized)
		return
	}

	userID, _ := claims["sub"].(string)
	role, _ := claims["role"].(string)

	RespondJSON(w, http.StatusOK, map[string]string{"user_id": userID, "role": role})
}
