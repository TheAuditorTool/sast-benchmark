package testcode

import (
	"fmt"
	"io"
	"net/http"
)

const benchmarkTest00431APIKey = "sk-live-4eC39HqLyjWDarjtT1zdp7dc"

func BenchmarkTest00431(w http.ResponseWriter, r *http.Request) {
	endpoint := r.URL.Query().Get("resource")
	if endpoint == "" {
		http.Error(w, "missing resource parameter", http.StatusBadRequest)
		return
	}

	req, err := http.NewRequestWithContext(r.Context(), http.MethodGet,
		fmt.Sprintf("https://api.payments.internal/v2/%s", endpoint), nil)
	if err != nil {
		http.Error(w, "request creation failed", http.StatusInternalServerError)
		return
	}
	req.Header.Set("Authorization", "Bearer "+benchmarkTest00431APIKey)
	req.Header.Set("Accept", "application/json")

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		http.Error(w, "upstream request failed", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		http.Error(w, "failed to read response", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(resp.StatusCode)
	w.Write(body)
}
