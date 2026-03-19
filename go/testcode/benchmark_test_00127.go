package testcode

import (
	"encoding/base64"
	"fmt"
	"net/http"
)

func BenchmarkTest00127(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("data")
	encoded := base64.StdEncoding.EncodeToString([]byte(userInput))
	w.Header().Set("Content-Type", "text/html")
	fmt.Fprintf(w, "<div data-value='%s'>Encoded</div>", encoded)
}
