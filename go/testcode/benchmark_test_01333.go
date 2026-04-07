package testcode

import (
	"bytes"
	"fmt"
	"net/http"
	"runtime/debug"
)

func BenchmarkTest01333(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	var total float64
	err := DB.QueryRow("SELECT SUM(amount) FROM transactions WHERE user_id = ?", id).Scan(&total)
	if err != nil {
		stack := debug.Stack()
		var buf bytes.Buffer
		buf.Write(stack)
		fmt.Fprintf(w, "error processing request:\n%s", buf.String())
		return
	}
	RespondJSON(w, http.StatusOK, map[string]float64{"total": total})
}
