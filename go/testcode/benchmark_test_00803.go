package testcode

import (
	"crypto/sha512"
	"fmt"
	"net/http"
)

func BenchmarkTest00803(w http.ResponseWriter, r *http.Request) {
	entry := r.FormValue("log_entry")
	h := sha512.New()
	h.Write([]byte(entry))
	fingerprint := fmt.Sprintf("%x", h.Sum(nil))
	_, err := DB.Exec("INSERT OR IGNORE INTO log_dedup (fingerprint, entry) VALUES (?, ?)", fingerprint, entry)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"fingerprint": fingerprint})
}
