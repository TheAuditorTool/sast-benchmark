package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
	"sync"
)

func BenchmarkTest00849(w http.ResponseWriter, r *http.Request) {
	var token string
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		b := make([]byte, 16)
		rand.Read(b)
		token = hex.EncodeToString(b)
	}()
	wg.Wait()
	http.SetCookie(w, &http.Cookie{
		Name:     "session",
		Value:    token,
		HttpOnly: true,
	})
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
