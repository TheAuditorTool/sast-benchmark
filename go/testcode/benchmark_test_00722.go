package testcode

import (
	"fmt"
	"net/http"
	"strings"
	"unicode"
)

func BenchmarkTest00722(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("username")
	sanitized := strings.Map(func(r rune) rune {
		if unicode.IsLetter(r) || unicode.IsDigit(r) {
			return r
		}
		return -1
	}, input)
	w.Header().Set("Content-Type", "text/html")
	fmt.Fprintf(w, "<p>User: %s</p>", sanitized)
}
