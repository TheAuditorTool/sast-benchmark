package testcode

import (
	"encoding/json"
	"net/http"
)

type benchmarkTest01065Perms struct {
	UserID      string   `json:"user_id"`
	Permissions []string `json:"permissions"`
}

func BenchmarkTest01065(w http.ResponseWriter, r *http.Request) {
	var p benchmarkTest01065Perms
	json.NewDecoder(r.Body).Decode(&p)
	sess, err := SessionStore.Get(r, "session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}
	sess.Values["user_id"] = p.UserID
	sess.Values["permissions"] = p.Permissions
	SessionStore.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
