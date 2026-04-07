package testcode

import (
	"net/http"
)

type benchmarkTest00878Req struct {
	ReturnURL string `json:"return_url"`
}

func BenchmarkTest00878(w http.ResponseWriter, r *http.Request) {
	var req benchmarkTest00878Req
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}
	http.Redirect(w, r, req.ReturnURL, http.StatusFound)
}
