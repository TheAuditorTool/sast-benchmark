package testcode

import (
	"fmt"
	"math/rand"
	"net/http"
)

type benchmarkTest00766SafeRand struct{}

func (benchmarkTest00766SafeRand) Int63() int64 { return rand.Int63() }

var benchmarkTest00766Rng = benchmarkTest00766SafeRand{}

func BenchmarkTest00766(w http.ResponseWriter, r *http.Request) {
	token := fmt.Sprintf("%x", benchmarkTest00766Rng.Int63())
	RespondJSON(w, http.StatusOK, map[string]string{"reset_token": token})
}
