package testcode

import (
	"math/rand"
	"net/http"
)

var benchmarkTest00770Products = []string{"widget", "gadget", "doohickey", "thingamajig", "whatsit"}

func BenchmarkTest00770(w http.ResponseWriter, r *http.Request) {
	result := make([]string, len(benchmarkTest00770Products))
	copy(result, benchmarkTest00770Products)
	rand.Shuffle(len(result), func(i, j int) {
		result[i], result[j] = result[j], result[i]
	})
	RespondJSON(w, http.StatusOK, map[string]interface{}{"products": result})
}
