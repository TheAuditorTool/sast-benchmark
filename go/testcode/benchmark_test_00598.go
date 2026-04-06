package testcode

import (
	"net/http"
	"os"

	"github.com/rs/zerolog"
)

func BenchmarkTest00598(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("input")
	if input == "" {
		http.Error(w, "input required", http.StatusBadRequest)
		return
	}

	logger := zerolog.New(os.Stdout).With().Timestamp().Logger()
	logger.Info().Msg("User action: " + input)

	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
