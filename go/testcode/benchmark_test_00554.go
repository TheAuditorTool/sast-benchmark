package testcode

import (
	"bytes"
	"encoding/json"
	"io"
	"net/http"
	"os"
)

func BenchmarkTest00554(w http.ResponseWriter, r *http.Request) {
	encryptedKeyPath := os.Getenv("ENCRYPTED_KEY_PATH")
	kmsEndpoint := os.Getenv("KMS_ENDPOINT")

	encryptedBlob, err := os.ReadFile(encryptedKeyPath)
	if err != nil {
		http.Error(w, "key file read error", http.StatusInternalServerError)
		return
	}

	kmsReqBody, _ := json.Marshal(map[string]string{
		"ciphertext": string(encryptedBlob),
	})

	resp, err := http.Post(kmsEndpoint+"/decrypt", "application/json", bytes.NewReader(kmsReqBody))
	if err != nil {
		http.Error(w, "kms unreachable", http.StatusServiceUnavailable)
		return
	}
	defer resp.Body.Close()

	body, _ := io.ReadAll(resp.Body)
	var kmsResp struct {
		Plaintext string `json:"plaintext"`
	}
	if err := json.Unmarshal(body, &kmsResp); err != nil {
		http.Error(w, "kms parse error", http.StatusInternalServerError)
		return
	}

	decryptedKey := []byte(kmsResp.Plaintext)
	_ = decryptedKey

	RespondJSON(w, http.StatusOK, map[string]string{"status": "key loaded"})
}
