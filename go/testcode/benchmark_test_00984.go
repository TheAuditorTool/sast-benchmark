package testcode

import (
	"net/http"

	"github.com/rs/zerolog/log"
)

func BenchmarkTest00984(w http.ResponseWriter, r *http.Request) {
	username := r.URL.Query().Get("username")
	action := r.URL.Query().Get("action")
	log.Info().
		Str("user", username).
		Str("action", action).
		Str("ip", r.RemoteAddr).
		Msg("audit")
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
