package testcode

import (
	"net/http"

	"github.com/rs/zerolog/log"
)

func BenchmarkTest00977(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	log.Error().Msg("login failed for user: " + username)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "error"})
}
