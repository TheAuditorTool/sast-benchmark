package testcode

import (
	"net/http"
)

func BenchmarkTest00741(w http.ResponseWriter, r *http.Request) {
	userURL := r.URL.Query().Get("link")
	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"provided_link": userURL,
		"message":       "link stored for display",
	})
}
