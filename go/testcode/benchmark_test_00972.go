package testcode

import (
	"log/slog"
	"net/http"
)

func BenchmarkTest00972(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("msg")
	slog.Info("event: " + userInput)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
