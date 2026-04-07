package testcode

import (
	"net/http"
	"sync"
)

func BenchmarkTest00880(w http.ResponseWriter, r *http.Request) {
	target := r.URL.Query().Get("redirect")
	var location string
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		location = target
	}()
	wg.Wait()
	http.Redirect(w, r, location, http.StatusFound)
}
