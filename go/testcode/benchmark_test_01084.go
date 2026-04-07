package testcode

import "net/http"

const benchmarkTest01084ESURL = "https://admin:ElasticPass123@es.example.com:9200"

func BenchmarkTest01084(w http.ResponseWriter, r *http.Request) {
	RespondJSON(w, http.StatusOK, map[string]string{"url": benchmarkTest01084ESURL})
}
