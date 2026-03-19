package testcode

import (
	"crypto/sha256"
	"fmt"
	"io"
	"net/http"
)

func BenchmarkTest00185(w http.ResponseWriter, r *http.Request) {
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "read failed", http.StatusBadRequest)
		return
	}

	hash := sha256.Sum256(body)
	cacheKey := fmt.Sprintf("content:%x", hash)

	var cached string
	err = DB.QueryRow("SELECT result FROM content_cache WHERE cache_key = ?", cacheKey).Scan(&cached)
	if err == nil {
		RespondJSON(w, http.StatusOK, map[string]string{
			"result": cached,
			"cache":  "hit",
		})
		return
	}

	result := fmt.Sprintf("processed_%x", hash[:8])
	_, _ = DB.Exec("INSERT INTO content_cache (cache_key, result) VALUES (?, ?)", cacheKey, result)

	RespondJSON(w, http.StatusOK, map[string]string{
		"result": result,
		"cache":  "miss",
	})
}
