package testcode

import "net/http"

func BenchmarkTest01094(w http.ResponseWriter, r *http.Request) {
	part1 := "secret"
	part2 := "_"
	part3 := "key_value"
	credential := part1 + part2 + part3
	RespondJSON(w, http.StatusOK, map[string]string{"cred": credential})
}
