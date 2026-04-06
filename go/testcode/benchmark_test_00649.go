package testcode

import (
	"crypto/tls"
	"io"
	"net"
	"net/http"
	"time"
)

func BenchmarkTest00649(w http.ResponseWriter, r *http.Request) {
	var req struct {
		URL string `json:"url"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	dialTLS := func(network, addr string) (net.Conn, error) {
		return tls.Dial(network, addr, &tls.Config{InsecureSkipVerify: true})
	}

	tr := &http.Transport{
		DialTLS:             dialTLS,
		TLSHandshakeTimeout: 10 * time.Second,
	}
	client := &http.Client{
		Timeout:   15 * time.Second,
		Transport: tr,
	}

	resp, err := client.Get(req.URL)
	if err != nil {
		http.Error(w, "upstream request failed", http.StatusBadGateway)
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
