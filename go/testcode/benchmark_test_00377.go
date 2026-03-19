package testcode

import (
	"fmt"
	"net/http"

	"github.com/jmoiron/sqlx"
)

var SqlxDB *sqlx.DB

func BenchmarkTest00377(w http.ResponseWriter, r *http.Request) {
	name := r.FormValue("name")
	SqlxDB.NamedExec(fmt.Sprintf("INSERT INTO users (name) VALUES ('%s')", name), map[string]interface{}{})
	RespondJSON(w, http.StatusOK, map[string]string{"status": "inserted"})
}
