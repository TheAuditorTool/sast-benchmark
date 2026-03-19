package testcode

import (
	"io"
	"net/http"
)

func BenchmarkTest00135(w http.ResponseWriter, r *http.Request) {
	var input struct {
		CallbackURL string `json:"callback_url"`
	}
	if err := ParseJSONBody(r, &input); err != nil {
		http.Error(w, "invalid body", http.StatusBadRequest)
		return
	}
	resp, err := http.Get(input.CallbackURL)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	RespondJSON(w, http.StatusOK, map[string]string{"callback_result": string(body)})
}
