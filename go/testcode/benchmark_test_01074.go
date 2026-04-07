package testcode

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
)

func BenchmarkTest01074(w http.ResponseWriter, r *http.Request) {
	b := make([]byte, 32)
	rand.Read(b)
	sess, _ := SessionStore.Get(r, "session")
	sess.Values["session_id"] = hex.EncodeToString(b)
	SessionStore.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
