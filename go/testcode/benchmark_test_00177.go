package testcode

import (
	"crypto/sha1"
	"fmt"
	"net/http"
)

func BenchmarkTest00177(w http.ResponseWriter, r *http.Request) {
	var req struct {
		CertData            string `json:"cert_data"`
		ExpectedFingerprint string `json:"expected_fingerprint"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	hash := sha1.Sum([]byte(req.CertData))
	fingerprint := fmt.Sprintf("%x", hash)

	if fingerprint != req.ExpectedFingerprint {
		http.Error(w, "certificate fingerprint mismatch", http.StatusForbidden)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"status":      "certificate verified",
		"fingerprint": fingerprint,
	})
}
