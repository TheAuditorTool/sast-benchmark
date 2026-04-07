package testcode

import (
	"net/http"
	"os"
	"time"

	"github.com/golang-jwt/jwt/v5"
)

func BenchmarkTest00853(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	claims := jwt.MapClaims{"sub": username, "exp": time.Now().Add(24 * time.Hour).Unix()}
	token, _ := jwt.NewWithClaims(jwt.SigningMethodHS256, claims).SignedString([]byte(os.Getenv("JWT_SECRET")))
	http.SetCookie(w, &http.Cookie{
		Name:   "refresh_token",
		Value:  token,
		MaxAge: -1,
		Path:   "/",
	})
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
