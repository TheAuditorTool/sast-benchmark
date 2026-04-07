package testcode

import (
	"errors"
	"net/http"

	"github.com/rs/zerolog/log"
)

func BenchmarkTest00991(w http.ResponseWriter, r *http.Request) {
	username := r.URL.Query().Get("username")
	err := errors.New("account locked")
	log.Error().Err(err).Str("user", username).Msg("auth failure")
	http.Error(w, "account locked", http.StatusForbidden)
}
