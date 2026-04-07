package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest00724(w http.ResponseWriter, r *http.Request) {
	var req struct {
		CallbackURL string `json:"callback_url"`
		Payload     string `json:"payload"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}
	resp, err := http.Post(req.CallbackURL, "application/json", strings.NewReader(req.Payload))
	if err != nil {
		http.Error(w, "callback error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": resp.Status})
}
