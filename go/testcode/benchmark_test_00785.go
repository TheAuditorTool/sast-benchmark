package testcode

import (
	"crypto/md5"
	"fmt"
	"net/http"
)

func BenchmarkTest00785(w http.ResponseWriter, r *http.Request) {
	body := r.FormValue("payload")
	secret := "api-secret-key"
	h := md5.Sum([]byte(secret + body))
	sig := fmt.Sprintf("%x", h)
	RespondJSON(w, http.StatusOK, map[string]string{"signature": sig})
}
