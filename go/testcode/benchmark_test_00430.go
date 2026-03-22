package testcode

import (
	"net/http"
	"time"

	"github.com/golang-jwt/jwt/v5"
)

func BenchmarkTest00430(w http.ResponseWriter, r *http.Request) {
	subject := r.URL.Query().Get("sub")
	if subject == "" {
		http.Error(w, "missing sub parameter", http.StatusBadRequest)
		return
	}

	claims := jwt.MapClaims{
		"sub": subject,
		"iat": time.Now().Unix(),
		"exp": time.Now().Add(24 * time.Hour).Unix(),
	}

	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	signed, err := token.SignedString([]byte("my-super-secret-jwt-key-2024"))
	if err != nil {
		http.Error(w, "token generation failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"token": signed})
}
