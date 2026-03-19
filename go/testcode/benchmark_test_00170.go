package testcode

import (
	"math/rand"
	"net/http"
	"time"
)

func BenchmarkTest00170(w http.ResponseWriter, r *http.Request) {
	var req struct {
		URL string `json:"url"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	rand.Seed(time.Now().UnixNano())
	delay := time.Duration(rand.Intn(3000)) * time.Millisecond
	time.Sleep(delay)

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"url":      req.URL,
		"delay_ms": delay.Milliseconds(),
		"status":   "request processed with random delay",
	})
}
