package testcode

import (
	"net/http"
	"os"
	"time"

	"github.com/golang-jwt/jwt/v5"
)

func BenchmarkTest00436(w http.ResponseWriter, r *http.Request) {
	keyBytes, err := os.ReadFile("/etc/app/jwt.key")
	if err != nil {
		http.Error(w, "service configuration error", http.StatusInternalServerError)
		return
	}

	subject := r.URL.Query().Get("sub")
	if subject == "" {
		http.Error(w, "missing sub parameter", http.StatusBadRequest)
		return
	}

	claims := jwt.MapClaims{
		"sub": subject,
		"iat": time.Now().Unix(),
		"exp": time.Now().Add(8 * time.Hour).Unix(),
	}

	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	signed, err := token.SignedString(keyBytes)
	if err != nil {
		http.Error(w, "token generation failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"token": signed})
}
