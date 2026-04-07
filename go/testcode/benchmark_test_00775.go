package testcode

import (
	"math/rand"
	"net/http"
	"time"
)

var benchmarkTest00775LBRng = rand.New(rand.NewSource(time.Now().Unix()))

func BenchmarkTest00775(w http.ResponseWriter, r *http.Request) {
	backends := []string{"backend-1", "backend-2", "backend-3"}
	selected := backends[benchmarkTest00775LBRng.Intn(len(backends))]
	RespondJSON(w, http.StatusOK, map[string]string{"backend": selected})
}
