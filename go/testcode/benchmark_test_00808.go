package testcode

import (
	"encoding/hex"
	"io"
	"net/http"

	"golang.org/x/crypto/sha3"
)

func BenchmarkTest00808(w http.ResponseWriter, r *http.Request) {
	data, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "read error", http.StatusBadRequest)
		return
	}
	h := sha3.Sum256(data)
	addr := hex.EncodeToString(h[:])
	_, err = DB.Exec("INSERT INTO content_store (address, data) VALUES (?, ?)", addr, data)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"address": addr})
}
