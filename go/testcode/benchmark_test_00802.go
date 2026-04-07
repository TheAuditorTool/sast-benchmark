package testcode

import (
	"crypto/md5"
	"fmt"
	"net/http"
)

func BenchmarkTest00802(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("q")
	raw := md5.Sum([]byte(input))
	cacheKey := fmt.Sprintf("cache:%x", raw)
	var result string
	err := DB.QueryRow("SELECT value FROM cache WHERE key = ?", cacheKey).Scan(&result)
	if err != nil {
		RespondJSON(w, http.StatusNotFound, map[string]string{"status": "miss"})
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"result": result})
}
