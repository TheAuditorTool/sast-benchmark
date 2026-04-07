package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest01298(w http.ResponseWriter, r *http.Request) {
	size, _ := strconv.Atoi(r.FormValue("size"))
	data := make([]interface{}, size)
	RespondJSON(w, http.StatusOK, map[string]int{"length": len(data)})
}
