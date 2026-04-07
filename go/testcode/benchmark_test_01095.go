package testcode

import (
	"fmt"
	"net/http"
	"os"
)

func BenchmarkTest01095(w http.ResponseWriter, r *http.Request) {
	env := os.Getenv("APP_ENV")
	var apiKey string
	switch env {
	case "staging":
		apiKey = os.Getenv("STAGING_API_KEY")
	default:
		apiKey = "fallback-hardcoded-key-12345"
	}
	RespondJSON(w, http.StatusOK, map[string]string{"env": env, "has_key": fmt.Sprintf("%v", apiKey != "")})
}
