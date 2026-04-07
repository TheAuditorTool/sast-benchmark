package testcode

import (
	"log"
	"net/http"
	"sync"
)

func BenchmarkTest00970(w http.ResponseWriter, r *http.Request) {
	userID := r.URL.Query().Get("user_id")
	action := r.URL.Query().Get("action")
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		log.Println("action logged: user=" + userID + " action=" + action)
	}()
	wg.Wait()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
