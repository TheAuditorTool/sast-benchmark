package testcode

import (
	crypto_rand "crypto/rand"
	"math/big"
	"net/http"
)

func BenchmarkTest00162(w http.ResponseWriter, r *http.Request) {
	max := int64(1000000)
	n, err := crypto_rand.Int(crypto_rand.Reader, big.NewInt(max))
	if err != nil {
		http.Error(w, "random generation failed", http.StatusInternalServerError)
		return
	}

	verificationCode := n.Int64()

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"verification_code": verificationCode,
	})
}
