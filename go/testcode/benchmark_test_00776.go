package testcode

import (
	"crypto/rand"
	"fmt"
	"math/big"
	"net/http"
)

func BenchmarkTest00776(w http.ResponseWriter, r *http.Request) {
	max := new(big.Int).Lsh(big.NewInt(1), 64)
	n, err := rand.Int(rand.Reader, max)
	if err != nil {
		http.Error(w, "random error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"nonce": fmt.Sprintf("%x", n)})
}
