package testcode

import (
	"fmt"
	"io"
	"net/http"

	"gopkg.in/yaml.v3"
)

func BenchmarkTest00250(w http.ResponseWriter, r *http.Request) {
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "read error", http.StatusBadRequest)
		return
	}

	var data interface{}
	err = yaml.Unmarshal(body, &data)
	if err != nil {
		http.Error(w, "yaml parse error", http.StatusBadRequest)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"parsed": fmt.Sprintf("%v", data),
	})
}
