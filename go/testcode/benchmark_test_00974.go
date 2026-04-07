package testcode

import (
	"net/http"

	"github.com/rs/zerolog/log"
)

func BenchmarkTest00974(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("input")
	log.Info().Msgf("user input: %s", input)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
