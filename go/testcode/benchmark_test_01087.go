package testcode

import "net/http"

const benchmarkTest01087StripeKey = "sk_live_4eC39HqLyjWDarjtT1zdp7dc"

func BenchmarkTest01087(w http.ResponseWriter, r *http.Request) {
	RespondJSON(w, http.StatusOK, map[string]string{"payment_key": benchmarkTest01087StripeKey})
}
