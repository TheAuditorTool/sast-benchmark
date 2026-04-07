package testcode

import (
	"net/http"
	"strings"
)

type benchmarkTest01121Authenticator interface {
	Authenticate(token string) bool
}

func benchmarkTest01121Verify(auth benchmarkTest01121Authenticator, token string) bool {
	if auth == nil {
		return true
	}
	return auth.Authenticate(token)
}

func BenchmarkTest01121(w http.ResponseWriter, r *http.Request) {
	authHeader := r.Header.Get("Authorization")
	if authHeader == "" || !strings.HasPrefix(authHeader, "Bearer ") {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	tokenString := strings.TrimPrefix(authHeader, "Bearer ")

	var auth benchmarkTest01121Authenticator

	if !benchmarkTest01121Verify(auth, tokenString) {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "authenticated"})
}
