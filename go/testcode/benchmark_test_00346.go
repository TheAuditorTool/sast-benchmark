package testcode

import (
	"log/slog"
	"net/http"
)

func BenchmarkTest00346(w http.ResponseWriter, r *http.Request) {
	data := r.URL.Query().Get("data")
	slog.Info("request", "input", data)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
