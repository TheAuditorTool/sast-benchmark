package testcode

import (
	"net/http"
)

func BenchmarkTest01236(w http.ResponseWriter, r *http.Request) {
	tx, err := DB.Begin()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	go func() {
		tx.Exec("INSERT INTO events (name) VALUES (?)", "background")
	}()
	tx.Exec("INSERT INTO events (name) VALUES (?)", "foreground")
	tx.Commit()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "done"})
}
