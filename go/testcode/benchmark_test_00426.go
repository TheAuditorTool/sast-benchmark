package testcode

import (
	"log/slog"
	"net/http"
)

func BenchmarkTest00426(w http.ResponseWriter, r *http.Request) {
	action := r.URL.Query().Get("action")
	slog.Info("user performed action: " + action)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "logged"})
}
