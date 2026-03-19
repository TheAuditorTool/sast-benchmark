package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00119(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("val")
	w.Header().Set("Content-Type", "text/html")
	fmt.Fprintf(w, "<script>var x='%s'</script>", userInput)
}
