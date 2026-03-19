package testcode

import (
	"crypto/rand"
	"net/http"
)

func BenchmarkTest00169(w http.ResponseWriter, r *http.Request) {
	var req struct {
		KeySize int `json:"key_size"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	if req.KeySize == 0 {
		req.KeySize = 256
	}

	prime, err := rand.Prime(rand.Reader, req.KeySize)
	if err != nil {
		http.Error(w, "prime generation failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"prime": prime.String(),
	})
}
