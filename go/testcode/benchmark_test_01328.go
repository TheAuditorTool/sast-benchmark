package testcode

import (
	"fmt"
	"net/http"
)

func benchmarkTest01328SendError(w http.ResponseWriter, err error) {
	fmt.Fprint(w, err.Error())
}

func BenchmarkTest01328(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	var name string
	err := DB.QueryRow("SELECT name FROM accounts WHERE id = ?", id).Scan(&name)
	if err != nil {
		benchmarkTest01328SendError(w, err)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"name": name})
}
