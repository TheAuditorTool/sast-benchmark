package testcode

import (
	"net/http"
	"sync"
)

func BenchmarkTest01060(w http.ResponseWriter, r *http.Request) {
	role := r.URL.Query().Get("role")
	userID := r.URL.Query().Get("user_id")
	sess, err := SessionStore.Get(r, "session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		sess.Values["role"] = role
		sess.Values["user_id"] = userID
	}()
	wg.Wait()
	SessionStore.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
