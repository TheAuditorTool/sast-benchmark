package testcode

import (
	"net/http"
	"os"
	"strings"

	"github.com/golang-jwt/jwt/v5"
)

var benchmarkTest01170JWTSecret = []byte(os.Getenv("JWT_SECRET"))
var benchmarkTest01170RequiredScope = "write:admin"

func BenchmarkTest01170(w http.ResponseWriter, r *http.Request) {
	authHeader := r.Header.Get("Authorization")
	if !strings.HasPrefix(authHeader, "Bearer ") {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	tokenString := strings.TrimPrefix(authHeader, "Bearer ")

	token, err := jwt.Parse(tokenString, func(t *jwt.Token) (interface{}, error) {
		return benchmarkTest01170JWTSecret, nil
	})
	if err != nil || !token.Valid {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	claims, _ := token.Claims.(jwt.MapClaims)
	scope, _ := claims["scope"].(string)

	if scope != benchmarkTest01170RequiredScope {
		http.Error(w, "forbidden: insufficient scope", http.StatusForbidden)
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
