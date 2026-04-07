package testcode

import (
	html_template "html/template"
	"net/http"
)

var benchmarkTest01227Allowed = map[string]bool{
	"index": true,
	"about": true,
	"help":  true,
}

func BenchmarkTest01227(w http.ResponseWriter, r *http.Request) {
	name := r.URL.Query().Get("page")
	if !benchmarkTest01227Allowed[name] {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	t, err := html_template.ParseFiles("templates/" + name + ".tmpl")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	t.Execute(w, map[string]string{"Page": name})
}
