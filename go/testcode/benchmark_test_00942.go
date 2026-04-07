package testcode

import (
	"bytes"
	"encoding/gob"
	"net/http"
)

func BenchmarkTest00942(w http.ResponseWriter, r *http.Request) {
	key := r.URL.Query().Get("key")
	var blob []byte
	err := DB.QueryRow("SELECT payload FROM cache WHERE key = ?", key).Scan(&blob)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	var obj interface{}
	if err := gob.NewDecoder(bytes.NewReader(blob)).Decode(&obj); err != nil {
		http.Error(w, "decode error", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{"data": obj})
}
