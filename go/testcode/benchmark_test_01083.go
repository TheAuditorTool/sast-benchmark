package testcode

import "net/http"

const benchmarkTest01083RedisAddr = "redis://:MyS3cr3tP@ssw0rd@localhost:6379"

func BenchmarkTest01083(w http.ResponseWriter, r *http.Request) {
	RespondJSON(w, http.StatusOK, map[string]string{"redis": benchmarkTest01083RedisAddr})
}
