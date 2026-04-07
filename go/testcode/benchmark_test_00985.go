package testcode

import (
	"log/slog"
	"net/http"
)

func BenchmarkTest00985(w http.ResponseWriter, r *http.Request) {
	username := r.URL.Query().Get("username")
	slog.LogAttrs(r.Context(), slog.LevelInfo, "user action",
		slog.String("user", username),
		slog.String("path", r.URL.Path),
	)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
