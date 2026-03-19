package testcode

import (
	"crypto/md5"
	"fmt"
	"net/http"
)

func BenchmarkTest00175(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Message   string `json:"message"`
		Signature string `json:"signature"`
		SecretKey string `json:"secret_key"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	data := req.SecretKey + req.Message
	hash := md5.Sum([]byte(data))
	computed := fmt.Sprintf("%x", hash)

	if computed != req.Signature {
		http.Error(w, "signature verification failed", http.StatusForbidden)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"status":  "verified",
		"message": req.Message,
	})
}
