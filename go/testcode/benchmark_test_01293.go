package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest01293(w http.ResponseWriter, r *http.Request) {
	arr := []string{"alpha", "beta", "gamma"}
	i, _ := strconv.Atoi(r.URL.Query().Get("idx"))
	result := arr[i]
	RespondJSON(w, http.StatusOK, map[string]string{"value": result})
}
