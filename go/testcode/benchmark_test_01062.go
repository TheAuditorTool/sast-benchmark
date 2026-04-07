package testcode

import (
	"encoding/json"
	"net/http"
)

func BenchmarkTest01062(w http.ResponseWriter, r *http.Request) {
	sess, err := SessionStore.Get(r, "session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}
	var data map[string]interface{}
	json.NewDecoder(r.Body).Decode(&data)
	for k, v := range data {
		sess.Values[k] = v
	}
	SessionStore.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
