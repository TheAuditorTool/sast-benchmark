package testcode

import (
	"net/http"
	"strconv"

	"github.com/rs/zerolog/log"
)

func BenchmarkTest00988(w http.ResponseWriter, r *http.Request) {
	userIDStr := r.URL.Query().Get("user_id")
	userID, _ := strconv.Atoi(userIDStr)
	log.Info().Int("user_id", userID).Str("path", r.URL.Path).Msg("request")
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
