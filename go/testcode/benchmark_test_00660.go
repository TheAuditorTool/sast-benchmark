package testcode

import (
	"net/http"
	"os"
	"strconv"
	"strings"
)

func BenchmarkTest00660(w http.ResponseWriter, r *http.Request) {
	counterFile := r.URL.Query().Get("file")
	if counterFile == "" {
		http.Error(w, "file parameter required", http.StatusBadRequest)
		return
	}

	go func() {
		data, err := os.ReadFile(counterFile)
		if err != nil {
			data = []byte("0")
		}

		n, err := strconv.Atoi(strings.TrimSpace(string(data)))
		if err != nil {
			n = 0
		}

		n++
		os.WriteFile(counterFile, []byte(strconv.Itoa(n)), 0644)
	}()

	RespondJSON(w, http.StatusAccepted, map[string]string{"status": "increment scheduled"})
}
