package testcode

import (
	"fmt"
	"math/rand"
	"net/http"
)

const benchmarkTest00762Mask = 0xDEADBEEF

func BenchmarkTest00762(w http.ResponseWriter, r *http.Request) {
	raw := rand.Int63()
	token := fmt.Sprintf("%x", raw^benchmarkTest00762Mask)
	RespondJSON(w, http.StatusOK, map[string]string{"session_token": token})
}
