package testcode

import (
	"net/http"
	"os"
	"strings"
)

func benchmarkTest01112LoadSecret(envVar, filePath string) string {
	if v := os.Getenv(envVar); v != "" {
		return v
	}
	data, err := os.ReadFile(filePath)
	if err != nil {
		return ""
	}
	return strings.TrimSpace(string(data))
}

func BenchmarkTest01112(w http.ResponseWriter, r *http.Request) {
	secret := benchmarkTest01112LoadSecret("APP_SECRET", "/run/secrets/app_secret")
	if secret == "" {
		http.Error(w, "secret not configured", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"configured": "true"})
}
