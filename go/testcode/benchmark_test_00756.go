package testcode

import (
	"fmt"
	"math/rand"
	"net/http"
)

var benchmarkTest00756Rng = rand.New(rand.NewSource(42))

func BenchmarkTest00756(w http.ResponseWriter, r *http.Request) {
	nonce := fmt.Sprintf("%x", benchmarkTest00756Rng.Int63())
	w.Header().Set("Content-Security-Policy", "script-src 'nonce-"+nonce+"'")
	RespondJSON(w, http.StatusOK, map[string]string{"nonce": nonce})
}
