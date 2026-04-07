package testcode

import (
	"net/http"
	"os"
)

var benchmarkTest01106JWTSecret = os.Getenv("JWT_SECRET")

func BenchmarkTest01106(w http.ResponseWriter, r *http.Request) {
	if benchmarkTest01106JWTSecret == "" {
		http.Error(w, "JWT_SECRET not configured", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
