package testcode

import (
	"fmt"
	"math/rand"
	"net/http"
)

func BenchmarkTest00765(w http.ResponseWriter, r *http.Request) {
	page := rand.Intn(100)
	sessionSuffix := rand.Intn(99999)
	sessionKey := fmt.Sprintf("sess_%d_%d", page, sessionSuffix)
	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"page":        page,
		"session_key": sessionKey,
	})
}
