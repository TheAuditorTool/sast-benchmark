package testcode

import (
	"net/http"
	"strings"
	"text/template"
)

type benchmarkTest00594Profile struct {
	Name  string `json:"name"`
	Email string `json:"email"`
}

var benchmarkTest00594Tmpl = template.Must(
	template.New("profile").Funcs(template.FuncMap{
		"upper": strings.ToUpper,
		"lower": strings.ToLower,
	}).Parse("Name: {{ upper .Name }}, Email: {{ lower .Email }}"),
)

func BenchmarkTest00594(w http.ResponseWriter, r *http.Request) {
	var profile benchmarkTest00594Profile
	if err := ParseJSONBody(r, &profile); err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}

	w.Header().Set("Content-Type", "text/plain")
	if err := benchmarkTest00594Tmpl.Execute(w, profile); err != nil {
		http.Error(w, "render error", http.StatusInternalServerError)
	}
}
