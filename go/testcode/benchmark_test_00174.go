package testcode

import (
	"crypto/sha1"
	"fmt"
	"net/http"
)

func BenchmarkTest00174(w http.ResponseWriter, r *http.Request) {
	var req struct {
		MasterSecret string `json:"master_secret"`
		Purpose      string `json:"purpose"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	h := sha1.New()
	h.Write([]byte(req.MasterSecret))
	h.Write([]byte(req.Purpose))
	derivedKey := h.Sum(nil)

	keyHex := fmt.Sprintf("%x", derivedKey)

	_, err := DB.Exec("INSERT INTO derived_keys (purpose, key_hash) VALUES (?, ?)",
		req.Purpose, keyHex)
	if err != nil {
		http.Error(w, "key derivation failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"derived_key": keyHex,
		"purpose":     req.Purpose,
	})
}
