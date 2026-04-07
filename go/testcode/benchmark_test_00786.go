package testcode

import (
	"crypto/hmac"
	"crypto/sha1"
	"fmt"
	"net/http"
)

var benchmarkTest00786Secret = []byte("webhook-secret")

func BenchmarkTest00786(w http.ResponseWriter, r *http.Request) {
	payload := r.FormValue("payload")
	mac := hmac.New(sha1.New, benchmarkTest00786Secret)
	mac.Write([]byte(payload))
	sig := fmt.Sprintf("sha1=%x", mac.Sum(nil))
	RespondJSON(w, http.StatusOK, map[string]string{"sig": sig})
}
