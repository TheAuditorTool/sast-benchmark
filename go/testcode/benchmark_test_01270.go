package testcode

import (
	"io"
	"net/http"
	"os"
)

func BenchmarkTest01270(w http.ResponseWriter, r *http.Request) {
	dest := r.URL.Query().Get("dest")
	if dest == "" {
		http.Error(w, "dest required", http.StatusBadRequest)
		return
	}

	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	go func() {
		f, err := os.Create(dest)
		if err != nil {
			return
		}
		defer f.Close()
		f.Write(body)
	}()

	RespondJSON(w, http.StatusAccepted, map[string]string{"queued": dest})
}
