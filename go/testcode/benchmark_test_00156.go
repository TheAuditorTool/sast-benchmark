package testcode

import (
	"math/rand"
	"net/http"
	"time"
)

func BenchmarkTest00156(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Purpose string `json:"purpose"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	rand.Seed(time.Now().UnixNano())
	charset := "abcdefghijklmnopqrstuvwxyz0123456789"
	chars := []byte(charset)
	rand.Shuffle(len(chars), func(i, j int) {
		chars[i], chars[j] = chars[j], chars[i]
	})
	randomID := string(chars[:16])

	_, err := DB.Exec("INSERT INTO identifiers (id, purpose) VALUES (?, ?)", randomID, req.Purpose)
	if err != nil {
		http.Error(w, "id creation failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"id":      randomID,
		"purpose": req.Purpose,
	})
}
