package testcode

import (
	"crypto/sha256"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest00029(w http.ResponseWriter, r *http.Request) {
	token := r.URL.Query().Get("token")
	hash := sha256.Sum256([]byte(token))
	hashStr := hex.EncodeToString(hash[:])
	rows, err := DB.Query("SELECT * FROM sessions WHERE token_hash = ?", hashStr)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
