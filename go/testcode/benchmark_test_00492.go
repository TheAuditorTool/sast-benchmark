package testcode

import (
	"net/http"
	"plugin"
)

func BenchmarkTest00492(w http.ResponseWriter, r *http.Request) {
	pluginPath := r.URL.Query().Get("plugin_path")
	if pluginPath == "" {
		http.Error(w, "missing plugin_path", http.StatusBadRequest)
		return
	}

	p, err := plugin.Open(pluginPath)
	if err != nil {
		http.Error(w, "plugin load error: "+err.Error(), http.StatusInternalServerError)
		return
	}

	sym, err := p.Lookup("Handler")
	if err != nil {
		http.Error(w, "handler not found in plugin", http.StatusInternalServerError)
		return
	}

	handler, ok := sym.(func(http.ResponseWriter, *http.Request))
	if !ok {
		http.Error(w, "invalid handler signature", http.StatusInternalServerError)
		return
	}

	handler(w, r)
}
