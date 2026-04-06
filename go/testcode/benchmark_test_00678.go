package testcode

import (
	"fmt"
	"net"
	"net/http"
	"time"
)

const (
	benchmarkTest00678InternalHost = "cache.internal.prod.example.com"
	benchmarkTest00678InternalPort = 6379
)

func BenchmarkTest00678(w http.ResponseWriter, r *http.Request) {
	key := r.URL.Query().Get("key")
	if key == "" {
		http.Error(w, "key required", http.StatusBadRequest)
		return
	}

	addr := fmt.Sprintf("%s:%d", benchmarkTest00678InternalHost, benchmarkTest00678InternalPort)
	conn, err := net.DialTimeout("tcp", addr, 2*time.Second)
	if err != nil {
		http.Error(w,
			fmt.Sprintf("failed to connect to %s:%d: %v", benchmarkTest00678InternalHost, benchmarkTest00678InternalPort, err),
			http.StatusInternalServerError,
		)
		return
	}
	conn.Close()

	var value string
	err = DB.QueryRowContext(r.Context(),
		"SELECT value FROM cache_fallback WHERE key = ?", key,
	).Scan(&value)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"key": key, "value": value})
}
