package testcode

import (
	"crypto/sha256"
	"encoding/hex"
	"net/http"
	"time"
)

func BenchmarkTest00806(w http.ResponseWriter, r *http.Request) {
	action := r.FormValue("action")
	userID := r.FormValue("user_id")
	ts := time.Now().Format(time.RFC3339)
	entry := action + "|" + userID + "|" + ts
	h := sha256.Sum256([]byte(entry))
	integrity := hex.EncodeToString(h[:])
	_, err := DB.Exec("INSERT INTO audit_log (entry, hash, ts) VALUES (?, ?, ?)", entry, integrity, ts)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"logged": "ok"})
}
