package testcode

import (
	"net/http"
	text_template "text/template"
)

type benchmarkTest01216Request struct {
	Template string `json:"template"`
	Name     string `json:"name"`
}

func BenchmarkTest01216(w http.ResponseWriter, r *http.Request) {
	var req benchmarkTest01216Request
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	t, err := text_template.New("").Parse(req.Template)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	t.Execute(w, map[string]string{"Name": req.Name})
}
