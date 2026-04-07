package testcode

import (
	"log/slog"
	"net/http"
)

func BenchmarkTest00978(w http.ResponseWriter, r *http.Request) {
	username := r.URL.Query().Get("username")
	slog.Info("login attempt", "user", username)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
