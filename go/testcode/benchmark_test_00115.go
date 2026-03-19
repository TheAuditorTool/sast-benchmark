package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00115(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("msg")
	w.Header().Set("Content-Type", "text/html")
	fmt.Fprintf(w, "<p>%s</p>", userInput)
}
