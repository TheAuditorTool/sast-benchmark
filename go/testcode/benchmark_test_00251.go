package testcode

import (
	"encoding/json"
	"io"
	"net/http"
	"reflect"
)

func BenchmarkTest00251(w http.ResponseWriter, r *http.Request) {
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "read error", http.StatusBadRequest)
		return
	}

	var data map[string]interface{}
	err = json.Unmarshal(body, &data)
	if err != nil {
		http.Error(w, "json parse error", http.StatusBadRequest)
		return
	}

	methodName, ok := data["method"].(string)
	if !ok {
		http.Error(w, "missing method field", http.StatusBadRequest)
		return
	}

	handler := &dynamicHandler{}
	val := reflect.ValueOf(handler)
	method := val.MethodByName(methodName)
	if !method.IsValid() {
		http.Error(w, "unknown method", http.StatusBadRequest)
		return
	}

	results := method.Call(nil)
	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"result": results[0].Interface(),
	})
}

type dynamicHandler struct{}

func (h *dynamicHandler) GetStatus() string {
	return "active"
}

func (h *dynamicHandler) GetVersion() string {
	return "1.0.0"
}
