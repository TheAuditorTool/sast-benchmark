package testcode

import (
	"net/http"

	"github.com/rs/zerolog/log"
)

func BenchmarkTest00497(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	action := r.FormValue("action")

	log.Info().
		Str("username", username).
		Str("action", action).
		Str("ip", r.RemoteAddr).
		Msg("login attempt")

	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
