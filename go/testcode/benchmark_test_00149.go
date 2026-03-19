package testcode

import (
	"io"
	"net/http"
	"strings"
)

func BenchmarkTest00149(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("message")
	payload := strings.NewReader(userInput)
	resp, err := http.Post("https://api.notifications.com/send", "application/json", payload)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	RespondJSON(w, http.StatusOK, map[string]string{"sent": string(body)})
}
