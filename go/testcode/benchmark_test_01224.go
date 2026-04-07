package testcode

import (
	"fmt"
	"net/http"
	"strconv"
	text_template "text/template"
)

var benchmarkTest01224Tmpl = text_template.Must(text_template.New("t").Funcs(text_template.FuncMap{
	"add": func(a, b int) int { return a + b },
	"mul": func(a, b int) int { return a * b },
}).Parse(`Result: {{add .X .Y}}, Product: {{mul .X .Y}}`))

type benchmarkTest01224Data struct{ X, Y int }

func BenchmarkTest01224(w http.ResponseWriter, r *http.Request) {
	x, _ := strconv.Atoi(r.URL.Query().Get("x"))
	y, _ := strconv.Atoi(r.URL.Query().Get("y"))
	if err := benchmarkTest01224Tmpl.Execute(w, benchmarkTest01224Data{X: x, Y: y}); err != nil {
		http.Error(w, fmt.Sprintf("render error: %v", err), http.StatusInternalServerError)
	}
}
