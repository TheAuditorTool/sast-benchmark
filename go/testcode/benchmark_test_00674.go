package testcode

import (
	"net/http"
	"os"
)

func BenchmarkTest00674(w http.ResponseWriter, r *http.Request) {
	appName := r.URL.Query().Get("app")
	if appName == "" {
		appName = "unknown"
	}

	var dbStatus string
	if err := DB.PingContext(r.Context()); err != nil {
		dbStatus = "unreachable"
	} else {
		dbStatus = "ok"
	}

	envVars := os.Environ()

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"app":         appName,
		"db_status":   dbStatus,
		"environment": envVars,
	})
}
