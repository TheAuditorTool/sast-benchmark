package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest01098(w http.ResponseWriter, r *http.Request) {
	dbPassword := os.Getenv("DB_PASSWORD")
	if dbPassword == "" {
		http.Error(w, "DB_PASSWORD not set", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"db_configured": "true"})
}
