package testcode

import (
	"fmt"
	"math/rand"
	"net/http"
)

func BenchmarkTest00767(w http.ResponseWriter, r *http.Request) {
	uploadToken := fmt.Sprintf("%x", rand.Uint64())
	RespondJSON(w, http.StatusOK, map[string]string{"upload_token": uploadToken})
}
