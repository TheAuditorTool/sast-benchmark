package testcode

import (
	"net/http"

	"github.com/gorilla/mux"
)

func BenchmarkTest00314(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	name := vars["name"]
	query := "SELECT * FROM users WHERE name = '" + name + "'"
	row := DB.QueryRow(query)
	var id int
	if err := row.Scan(&id); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{"id": id})
}
