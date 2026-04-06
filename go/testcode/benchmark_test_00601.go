package testcode

import (
	"net/http"
	"os"

	"github.com/rs/zerolog"
)

func BenchmarkTest00601(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("input")
	if input == "" {
		http.Error(w, "input required", http.StatusBadRequest)
		return
	}

	logger := zerolog.New(os.Stdout).With().Timestamp().Logger()
	logger.Info().Str("action", input).Msg("user action")

	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
