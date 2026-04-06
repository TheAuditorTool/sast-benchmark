package testcode

import (
	"encoding/base64"
	"net/http"
)

func BenchmarkTest00538(w http.ResponseWriter, r *http.Request) {
	var body struct {
		UserID    int    `json:"user_id"`
		Plaintext string `json:"plaintext"`
	}
	if err := ParseJSONBody(r, &body); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	encoded := base64.StdEncoding.EncodeToString([]byte(body.Plaintext))

	_, err := DB.Exec(
		"INSERT INTO encrypted_values (user_id, value) VALUES (?, ?)",
		body.UserID, encoded,
	)
	if err != nil {
		http.Error(w, "storage error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"encrypted": encoded,
	})
}
