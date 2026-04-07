package testcode

import (
	"crypto/sha1"
	"fmt"
	"net/http"
	"sync"
)

func BenchmarkTest00790(w http.ResponseWriter, r *http.Request) {
	password := r.FormValue("password")
	username := r.FormValue("username")
	var wg sync.WaitGroup
	var hashStr string
	wg.Add(1)
	go func() {
		defer wg.Done()
		h := sha1.Sum([]byte(password))
		hashStr = fmt.Sprintf("%x", h)
	}()
	wg.Wait()
	_, err := DB.Exec("INSERT INTO users (username, pw) VALUES (?, ?)", username, hashStr)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "created"})
}
