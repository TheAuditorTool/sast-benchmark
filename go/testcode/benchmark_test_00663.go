package testcode

import (
	"net/http"
)

type benchmarkTest00663Svc struct {
	endpoint string
	timeout  int
}

func benchmarkTest00663NewSvc() *benchmarkTest00663Svc {
	return &benchmarkTest00663Svc{
		endpoint: "http://internal-api:8080",
		timeout:  30,
	}
}

var benchmarkTest00663Service *benchmarkTest00663Svc

func BenchmarkTest00663(w http.ResponseWriter, r *http.Request) {
	if benchmarkTest00663Service == nil {
		benchmarkTest00663Service = benchmarkTest00663NewSvc()
	}

	action := r.URL.Query().Get("action")
	if action == "" {
		http.Error(w, "action required", http.StatusBadRequest)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"action":   action,
		"endpoint": benchmarkTest00663Service.endpoint,
		"timeout":  benchmarkTest00663Service.timeout,
	})
}
