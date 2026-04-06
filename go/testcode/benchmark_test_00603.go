package testcode

import (
	"log/slog"
	"net/http"
	"strings"
)

type benchmarkTest00603SanitizedString string

func (s benchmarkTest00603SanitizedString) LogValue() slog.Value {
	clean := strings.ReplaceAll(string(s), "\n", " ")
	clean = strings.ReplaceAll(clean, "\r", " ")
	return slog.StringValue(clean)
}

func BenchmarkTest00603(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("input")
	if input == "" {
		http.Error(w, "input required", http.StatusBadRequest)
		return
	}

	slog.Info("request", "input", benchmarkTest00603SanitizedString(input))

	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
