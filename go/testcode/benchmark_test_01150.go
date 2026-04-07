package testcode

import (
	"net/http"
	"sync"
)

var benchmarkTest01150Cache sync.Map

func BenchmarkTest01150(w http.ResponseWriter, r *http.Request) {
	cacheKey := r.Header.Get("X-Cache-Key")
	userID := r.Header.Get("X-User-ID")

	if cached, ok := benchmarkTest01150Cache.Load(cacheKey); ok {
		role, _ := cached.(string)
		if role != "admin" {
			http.Error(w, "forbidden", http.StatusForbidden)
			return
		}
		rows, err := DB.Query("SELECT id, email FROM users WHERE id = ?", userID)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		defer rows.Close()
		RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
		return
	}

	var role string
	DB.QueryRow("SELECT role FROM users WHERE id = ?", userID).Scan(&role)
	benchmarkTest01150Cache.Store(cacheKey, role)

	if role != "admin" {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
