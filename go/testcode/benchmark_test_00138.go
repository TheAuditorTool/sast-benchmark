package testcode

import (
	"io"
	"net/http"
	"net/url"
)

func BenchmarkTest00138(w http.ResponseWriter, r *http.Request) {
	userHost := r.URL.Query().Get("host")
	u := &url.URL{
		Scheme: "http",
		Host:   userHost,
		Path:   "/api/info",
	}
	resp, err := http.Get(u.String())
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	RespondJSON(w, http.StatusOK, map[string]string{"info": string(body)})
}
