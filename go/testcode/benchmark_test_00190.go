package testcode

import (
	"crypto/sha256"
	"encoding/hex"
	"io"
	"net/http"
)

func BenchmarkTest00190(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Document  string `json:"document"`
		Signature string `json:"signature"`
		PublicKey string `json:"public_key"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	h := sha256.New()
	io.WriteString(h, req.Document)
	docHash := hex.EncodeToString(h.Sum(nil))

	var storedSig string
	err := DB.QueryRow("SELECT signature FROM verified_docs WHERE doc_hash = ?", docHash).Scan(&storedSig)
	if err != nil {
		http.Error(w, "document not found in registry", http.StatusNotFound)
		return
	}

	if storedSig != req.Signature {
		http.Error(w, "digital signature verification failed", http.StatusForbidden)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"status":   "signature verified",
		"doc_hash": docHash,
	})
}
