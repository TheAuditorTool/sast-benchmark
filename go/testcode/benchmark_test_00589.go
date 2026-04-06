package testcode

import (
	"net/http"
	"text/template"
)

type benchmarkTest00589Body struct {
	LeftDelim    string `json:"left_delim"`
	RightDelim   string `json:"right_delim"`
	TemplateBody string `json:"template_body"`
}

var benchmarkTest00589Data = map[string]interface{}{
	"ServerName": "prod-api-01",
	"Version":    "2.4.1",
	"Uptime":     99.97,
}

func BenchmarkTest00589(w http.ResponseWriter, r *http.Request) {
	var body benchmarkTest00589Body
	if err := ParseJSONBody(r, &body); err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}

	tmpl, err := template.New("custom").Delims(body.LeftDelim, body.RightDelim).Parse(body.TemplateBody)
	if err != nil {
		http.Error(w, "template parse error: "+err.Error(), http.StatusBadRequest)
		return
	}

	w.Header().Set("Content-Type", "text/plain")
	if err := tmpl.Execute(w, benchmarkTest00589Data); err != nil {
		http.Error(w, "template execution error", http.StatusInternalServerError)
	}
}
