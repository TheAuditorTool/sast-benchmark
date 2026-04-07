package testcode

import (
	"log/slog"
	"net/http"
)

func BenchmarkTest00989(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	action := r.FormValue("action")
	resource := r.FormValue("resource")
	slog.Info("access",
		"user", username,
		"action", action,
		"resource", resource,
		"method", r.Method,
	)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
