package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest01111(w http.ResponseWriter, r *http.Request) {
	secret := os.Getenv("JWT_SECRET")
	if len(secret) < 32 {
		http.Error(w, "JWT_SECRET too short or not set", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"jwt_configured": "true"})
}
