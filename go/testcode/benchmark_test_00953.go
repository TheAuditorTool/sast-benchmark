package testcode

import (
	"encoding/gob"
	"net/http"
)

type benchmarkTest00953Event struct {
	EventType string
	UserID    int
	Timestamp int64
}

func BenchmarkTest00953(w http.ResponseWriter, r *http.Request) {
	var evt benchmarkTest00953Event
	if err := gob.NewDecoder(r.Body).Decode(&evt); err != nil {
		http.Error(w, "decode error", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"type": evt.EventType,
		"uid":  evt.UserID,
	})
}
