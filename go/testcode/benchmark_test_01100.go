package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest01100(w http.ResponseWriter, r *http.Request) {
	apiKey, ok := os.LookupEnv("API_KEY")
	if !ok || apiKey == "" {
		http.Error(w, "API_KEY environment variable not set", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"api_configured": "true"})
}
