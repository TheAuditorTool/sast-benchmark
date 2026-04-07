package testcode

import (
	"net/http"

	"github.com/rs/zerolog/log"
)

func BenchmarkTest00966(w http.ResponseWriter, r *http.Request) {
	searchTerm := r.URL.Query().Get("q")
	log.Info().Msg("search: " + searchTerm)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
