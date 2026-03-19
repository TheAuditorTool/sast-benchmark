package testcode

import (
	"crypto/md5"
	"fmt"
	"net/http"
)

func BenchmarkTest00188(w http.ResponseWriter, r *http.Request) {
	key := r.URL.Query().Get("key")
	if key == "" {
		http.Error(w, "key required", http.StatusBadRequest)
		return
	}

	hash := md5.Sum([]byte(key))
	cacheKey := fmt.Sprintf("cache:%x", hash)

	var value string
	err := DB.QueryRow("SELECT value FROM kv_cache WHERE cache_key = ?", cacheKey).Scan(&value)
	if err == nil {
		RespondJSON(w, http.StatusOK, map[string]string{
			"value": value,
			"cache": "hit",
		})
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"cache": "miss",
	})
}
