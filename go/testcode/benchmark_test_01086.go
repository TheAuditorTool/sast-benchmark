package testcode

import "net/http"

const (
	benchmarkTest01086AWSAccessKey = "AKIAIOSFODNN7EXAMPLE"
	benchmarkTest01086AWSSecret    = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
)

func BenchmarkTest01086(w http.ResponseWriter, r *http.Request) {
	RespondJSON(w, http.StatusOK, map[string]string{
		"key_id": benchmarkTest01086AWSAccessKey,
	})
}
