package testcode

import (
	"net/http"
	"os"
	"text/template"
)

type benchmarkTest00591Body struct {
	TemplateStr string `json:"template"`
}

func BenchmarkTest00591(w http.ResponseWriter, r *http.Request) {
	var body benchmarkTest00591Body
	if err := ParseJSONBody(r, &body); err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}

	funcMap := template.FuncMap{
		"readfile": func(path string) string {
			data, _ := os.ReadFile(path)
			return string(data)
		},
	}

	tmpl, err := template.New("user").Funcs(funcMap).Parse(body.TemplateStr)
	if err != nil {
		http.Error(w, "template parse error", http.StatusBadRequest)
		return
	}

	w.Header().Set("Content-Type", "text/plain")
	if err := tmpl.Execute(w, nil); err != nil {
		http.Error(w, "execution error", http.StatusInternalServerError)
	}
}
