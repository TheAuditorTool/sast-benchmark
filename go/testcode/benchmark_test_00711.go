package testcode

import (
	"net/http"
	"text/template"
)

func BenchmarkTest00711(w http.ResponseWriter, r *http.Request) {
	greeting := r.URL.Query().Get("greeting")
	tmplStr := "<html><body>" + greeting + " {{.Name}}</body></html>"
	t, err := template.New("page").Parse(tmplStr)
	if err != nil {
		http.Error(w, "template error", http.StatusInternalServerError)
		return
	}
	w.Header().Set("Content-Type", "text/html")
	t.Execute(w, struct{ Name string }{Name: "User"})
}
