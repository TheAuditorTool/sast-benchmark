package testcode

import (
	"fmt"
	"net/http"
	"os"
)

func BenchmarkTest00975(w http.ResponseWriter, r *http.Request) {
	filename := r.URL.Query().Get("file")
	userID := r.URL.Query().Get("user_id")
	entry := fmt.Sprintf("file_access file=%s user=%s\n", filename, userID)
	os.Stderr.WriteString(entry)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
