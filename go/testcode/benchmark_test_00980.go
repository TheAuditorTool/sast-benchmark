package testcode

import (
	"net/http"

	"github.com/rs/zerolog/log"
)

func BenchmarkTest00980(w http.ResponseWriter, r *http.Request) {
	username := r.URL.Query().Get("username")
	log.Info().Str("user", username).Msg("login attempt")
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
