package testcode

import (
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00198(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Plaintext string `json:"plaintext"`
		Shift     int    `json:"shift"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	if req.Shift == 0 {
		req.Shift = 13
	}

	data := []byte(req.Plaintext)
	for i, b := range data {
		if b >= 'a' && b <= 'z' {
			data[i] = 'a' + (b-'a'+byte(req.Shift))%26
		} else if b >= 'A' && b <= 'Z' {
			data[i] = 'A' + (b-'A'+byte(req.Shift))%26
		}
	}

	_, err := DB.Exec("INSERT INTO encrypted_messages (ciphertext, algorithm) VALUES (?, ?)",
		hex.EncodeToString(data), "caesar")
	if err != nil {
		http.Error(w, "storage failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"ciphertext": string(data),
		"algorithm":  "caesar",
	})
}
