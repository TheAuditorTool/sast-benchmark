package testcode

import (
	"log/slog"
	"net/http"
	"strings"
)

func BenchmarkTest00992(w http.ResponseWriter, r *http.Request) {
	rawInput := r.URL.Query().Get("note")
	sanitized := strings.ReplaceAll(rawInput, "\n", " ")
	sanitized = strings.ReplaceAll(sanitized, "\r", " ")
	slog.Info("note recorded", "note", sanitized)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
