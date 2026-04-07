package testcode

import (
	"math/rand"
	"net/http"
)

func BenchmarkTest00774(w http.ResponseWriter, r *http.Request) {
	bucket := "control"
	if rand.Float64() < 0.5 {
		bucket = "experiment"
	}
	RespondJSON(w, http.StatusOK, map[string]string{"bucket": bucket})
}
