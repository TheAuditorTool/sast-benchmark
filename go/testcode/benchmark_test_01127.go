package testcode

import (
	"net/http"
	"sync"
)

var benchmarkTest01127mu sync.Mutex
var benchmarkTest01127tokens = map[string]string{}

func BenchmarkTest01127(w http.ResponseWriter, r *http.Request) {
	token := r.Header.Get("X-Auth-Token")
	action := r.URL.Query().Get("action")

	benchmarkTest01127mu.Lock()
	userID, exists := benchmarkTest01127tokens[token]
	benchmarkTest01127mu.Unlock()

	if !exists {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	benchmarkTest01127mu.Lock()
	_, stillValid := benchmarkTest01127tokens[token]
	benchmarkTest01127mu.Unlock()

	RespondJSON(w, http.StatusOK, map[string]string{"user_id": userID, "action": action, "valid": func() string {
		if stillValid {
			return "true"
		}
		return "false"
	}()})
}
