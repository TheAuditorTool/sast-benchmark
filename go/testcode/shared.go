package testcode

import (
	"database/sql"
	"encoding/json"
	"net/http"
)

// DB is the shared database connection for all benchmark tests.
// Injected at startup via SetDB.
var DB *sql.DB

// SetDB sets the shared database connection.
func SetDB(db *sql.DB) {
	DB = db
}

// RespondJSON writes a JSON response with the given status code.
func RespondJSON(w http.ResponseWriter, status int, data interface{}) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	json.NewEncoder(w).Encode(data)
}

// ParseJSONBody decodes the request body into the given struct.
func ParseJSONBody(r *http.Request, v interface{}) error {
	return json.NewDecoder(r.Body).Decode(v)
}
