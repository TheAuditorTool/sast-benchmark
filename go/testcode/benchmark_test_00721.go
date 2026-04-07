package testcode

import (
	"html/template"
	"net/http"
)

func BenchmarkTest00721(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("note")
	escaped := template.HTMLEscapeString(input)
	RespondJSON(w, http.StatusOK, map[string]string{"note": escaped})
}
