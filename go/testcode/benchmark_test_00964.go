package testcode

import (
	"fmt"
	"net/http"
	"os"
)

func BenchmarkTest00964(w http.ResponseWriter, r *http.Request) {
	userAgent := r.Header.Get("User-Agent")
	fmt.Fprintf(os.Stderr, "request: "+userAgent+"\n")
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
