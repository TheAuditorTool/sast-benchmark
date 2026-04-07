package testcode

import (
	"net/http"
	"os"
	"strings"
)

func BenchmarkTest01109(w http.ResponseWriter, r *http.Request) {
	secretPath := "/run/secrets/db_password"
	data, err := os.ReadFile(secretPath)
	if err != nil {
		secretPath = os.Getenv("DB_SECRET_PATH")
		data, err = os.ReadFile(secretPath)
		if err != nil {
			http.Error(w, "secret not available", http.StatusInternalServerError)
			return
		}
	}
	password := strings.TrimSpace(string(data))
	RespondJSON(w, http.StatusOK, map[string]bool{"db_configured": password != ""})
}
