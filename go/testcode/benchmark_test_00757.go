package testcode

import (
	"encoding/hex"
	"math/rand"
	"net/http"
)

func BenchmarkTest00757(w http.ResponseWriter, r *http.Request) {
	iv := make([]byte, 16)
	rng := rand.New(rand.NewSource(42))
	rng.Read(iv)
	RespondJSON(w, http.StatusOK, map[string]string{"iv": hex.EncodeToString(iv)})
}
