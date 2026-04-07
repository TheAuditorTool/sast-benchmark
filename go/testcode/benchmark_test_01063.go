package testcode

import "net/http"

func BenchmarkTest01063(w http.ResponseWriter, r *http.Request) {
	sess, err := SessionStore.Get(r, "session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}
	for key, vals := range r.Header {
		if len(vals) > 0 {
			sess.Values[key] = vals[0]
		}
	}
	SessionStore.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
