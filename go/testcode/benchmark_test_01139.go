package testcode

import (
	"net/http"
	"sync"
	"time"

	"golang.org/x/crypto/bcrypt"
)

var benchmarkTest01139mu sync.Mutex
var benchmarkTest01139attempts = map[string][]time.Time{}

func BenchmarkTest01139(w http.ResponseWriter, r *http.Request) {
	ip := r.RemoteAddr
	const maxAttempts = 5
	const window = time.Minute

	benchmarkTest01139mu.Lock()
	now := time.Now()
	recent := benchmarkTest01139attempts[ip][:0]
	for _, t := range benchmarkTest01139attempts[ip] {
		if now.Sub(t) < window {
			recent = append(recent, t)
		}
	}
	benchmarkTest01139attempts[ip] = recent
	if len(recent) >= maxAttempts {
		benchmarkTest01139mu.Unlock()
		http.Error(w, "too many requests", http.StatusTooManyRequests)
		return
	}
	benchmarkTest01139mu.Unlock()

	username := r.FormValue("username")
	password := r.FormValue("password")

	var hashedPassword string
	row := DB.QueryRow("SELECT password FROM users WHERE username = ?", username)
	if err := row.Scan(&hashedPassword); err != nil {
		benchmarkTest01139mu.Lock()
		benchmarkTest01139attempts[ip] = append(benchmarkTest01139attempts[ip], now)
		benchmarkTest01139mu.Unlock()
		http.Error(w, "invalid credentials", http.StatusUnauthorized)
		return
	}

	if err := bcrypt.CompareHashAndPassword([]byte(hashedPassword), []byte(password)); err != nil {
		benchmarkTest01139mu.Lock()
		benchmarkTest01139attempts[ip] = append(benchmarkTest01139attempts[ip], now)
		benchmarkTest01139mu.Unlock()
		http.Error(w, "invalid credentials", http.StatusUnauthorized)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "authenticated"})
}
