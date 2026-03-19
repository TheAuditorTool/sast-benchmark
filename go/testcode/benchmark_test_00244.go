package testcode

import (
	"io"
	"net/http"
	"time"
)

func BenchmarkTest00244(w http.ResponseWriter, r *http.Request) {
	var req struct {
		URL string `json:"url"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	client := &http.Client{
		Timeout: 10 * time.Second,
	}

	resp, err := client.Get(req.URL)
	if err != nil {
		http.Error(w, "upstream error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		http.Error(w, "read error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"status": resp.StatusCode,
		"body":   string(body),
	})
}
