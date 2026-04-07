package testcode

import (
	"encoding/gob"
	"net/http"
)

type benchmarkTest00939Container struct {
	Payload interface{}
}

func (c *benchmarkTest00939Container) Process() string {
	if s, ok := c.Payload.(string); ok {
		return s
	}
	return ""
}

func BenchmarkTest00939(w http.ResponseWriter, r *http.Request) {
	c := &benchmarkTest00939Container{}
	if err := gob.NewDecoder(r.Body).Decode(&c.Payload); err != nil {
		http.Error(w, "decode error", http.StatusBadRequest)
		return
	}
	result := c.Process()
	RespondJSON(w, http.StatusOK, map[string]string{"result": result})
}
