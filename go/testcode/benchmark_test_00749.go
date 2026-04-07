package testcode

import (
	"fmt"
	"net/http"
	"net/url"
)

func BenchmarkTest00749(w http.ResponseWriter, r *http.Request) {
	searchQuery := r.URL.Query().Get("q")
	target := fmt.Sprintf("https://search.internal.example.com/api?q=%s", url.QueryEscape(searchQuery))
	resp, err := http.Get(target)
	if err != nil {
		http.Error(w, "fetch error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
