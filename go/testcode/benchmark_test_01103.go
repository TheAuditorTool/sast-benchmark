package testcode

import (
	"net/http"
	"os"
	"strings"
)

func BenchmarkTest01103(w http.ResponseWriter, r *http.Request) {
	keyFile := os.Getenv("SECRET_KEY_FILE")
	if keyFile == "" {
		http.Error(w, "SECRET_KEY_FILE not set", http.StatusInternalServerError)
		return
	}
	data, err := os.ReadFile(keyFile)
	if err != nil {
		http.Error(w, "key file not found", http.StatusInternalServerError)
		return
	}
	key := strings.TrimSpace(string(data))
	RespondJSON(w, http.StatusOK, map[string]bool{"key_loaded": key != ""})
}
