package testcode

import (
	"encoding/json"
	"net/http"
	"os"
	"path/filepath"
)

type benchmarkTest01105Credentials struct {
	Token string `json:"token"`
}

func BenchmarkTest01105(w http.ResponseWriter, r *http.Request) {
	home, err := os.UserHomeDir()
	if err != nil {
		http.Error(w, "home dir error", http.StatusInternalServerError)
		return
	}
	credFile := filepath.Join(home, ".config", "myapp", "credentials.json")
	f, err := os.Open(credFile)
	if err != nil {
		http.Error(w, "credentials not found", http.StatusUnauthorized)
		return
	}
	defer f.Close()
	var creds benchmarkTest01105Credentials
	json.NewDecoder(f).Decode(&creds)
	RespondJSON(w, http.StatusOK, map[string]bool{"authenticated": creds.Token != ""})
}
