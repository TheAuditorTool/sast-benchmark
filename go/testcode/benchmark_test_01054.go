package testcode

import "net/http"

func BenchmarkTest01054(w http.ResponseWriter, r *http.Request) {
	sess, err := SessionStore.Get(r, "session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}
	c, err := r.Cookie("uid")
	if err == nil {
		sess.Values["user_id"] = c.Value
	}
	SessionStore.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
