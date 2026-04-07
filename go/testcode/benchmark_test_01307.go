package testcode

import (
	"net/http"
)

func BenchmarkTest01307(w http.ResponseWriter, r *http.Request) {
	var m map[string]string
	if r.FormValue("init") == "" {
		m[r.FormValue("key")] = "val"
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
