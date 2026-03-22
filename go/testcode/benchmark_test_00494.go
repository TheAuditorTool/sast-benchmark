package testcode

import (
	"net/http"
	"plugin"
)

func BenchmarkTest00494(w http.ResponseWriter, r *http.Request) {
	p, err := plugin.Open("/usr/lib/plugins/handler.so")
	if err != nil {
		http.Error(w, "plugin load error", http.StatusInternalServerError)
		return
	}

	sym, err := p.Lookup("Process")
	if err != nil {
		http.Error(w, "symbol not found", http.StatusInternalServerError)
		return
	}

	processFn, ok := sym.(func(string) string)
	if !ok {
		http.Error(w, "invalid function signature", http.StatusInternalServerError)
		return
	}

	input := r.FormValue("input")
	result := processFn(input)
	RespondJSON(w, http.StatusOK, map[string]string{"result": result})
}
