package testcode

import "net/http"

func benchmarkTest01092GetAuthToken() string {
	return "Bearer secret-api-token-xyz789"
}

func BenchmarkTest01092(w http.ResponseWriter, r *http.Request) {
	token := benchmarkTest01092GetAuthToken()
	RespondJSON(w, http.StatusOK, map[string]string{"token": token})
}
