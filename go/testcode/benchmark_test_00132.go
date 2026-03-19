package testcode

import (
	"io"
	"net/http"
	"strings"
)

func BenchmarkTest00132(w http.ResponseWriter, r *http.Request) {
	userURL := r.URL.Query().Get("target")
	resp, err := http.Post(userURL, "application/json", strings.NewReader("{}"))
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	RespondJSON(w, http.StatusOK, map[string]string{"result": string(body)})
}
