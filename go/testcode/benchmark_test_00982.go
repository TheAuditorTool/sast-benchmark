package testcode

import (
	"log"
	"net/http"
	"strings"
)

func BenchmarkTest00982(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("input")
	safe := strings.Map(func(c rune) rune {
		if c == '\n' || c == '\r' {
			return ' '
		}
		return c
	}, input)
	log.Println("search:", safe)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
