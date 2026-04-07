package testcode

import (
	"net/http"
)

func BenchmarkTest00731(w http.ResponseWriter, r *http.Request) {
	targetURL := r.URL.Query().Get("url")
	resultCh := make(chan string, 1)

	go func() {
		resp, err := http.Get(targetURL)
		if err != nil {
			resultCh <- "error"
			return
		}
		defer resp.Body.Close()
		resultCh <- resp.Status
	}()

	status := <-resultCh
	RespondJSON(w, http.StatusOK, map[string]string{"status": status})
}
