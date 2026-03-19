package testcode

import (
	"encoding/base64"
	"math/rand"
	"net/http"
)

func BenchmarkTest00152(w http.ResponseWriter, r *http.Request) {
	var req struct {
		KeyName string `json:"key_name"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	buf := make([]byte, 32)
	rand.Read(buf)
	keyMaterial := base64.StdEncoding.EncodeToString(buf)

	_, err := DB.Exec("INSERT INTO encryption_keys (name, material) VALUES (?, ?)",
		req.KeyName, keyMaterial)
	if err != nil {
		http.Error(w, "key storage failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"key_name": req.KeyName,
		"key":      keyMaterial,
	})
}
