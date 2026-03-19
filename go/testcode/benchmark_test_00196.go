package testcode

import (
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00196(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Plaintext string `json:"plaintext"`
		Key       string `json:"key"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	data := []byte(req.Plaintext)
	key := []byte(req.Key)

	for i := range data {
		data[i] ^= key[i%len(key)]
	}

	_, err := DB.Exec("INSERT INTO encrypted_messages (ciphertext, key_id) VALUES (?, ?)",
		hex.EncodeToString(data), "xor-key-1")
	if err != nil {
		http.Error(w, "storage failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"ciphertext": hex.EncodeToString(data),
		"algorithm":  "xor",
	})
}
