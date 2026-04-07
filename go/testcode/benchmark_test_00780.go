package testcode

import (
	"crypto/rand"
	"encoding/base64"
	"net/http"
)

func BenchmarkTest00780(w http.ResponseWriter, r *http.Request) {
	b := make([]byte, 40)
	if _, err := rand.Read(b); err != nil {
		http.Error(w, "random error", http.StatusInternalServerError)
		return
	}
	apiKey := base64.URLEncoding.EncodeToString(b)
	userID := r.URL.Query().Get("user_id")
	_, _ = DB.Exec("UPDATE users SET api_key = ? WHERE id = ?", apiKey, userID)
	RespondJSON(w, http.StatusOK, map[string]string{"api_key": apiKey})
}
