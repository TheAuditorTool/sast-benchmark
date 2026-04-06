package testcode

import (
	"log/slog"
	"net/http"
)

func BenchmarkTest00600(w http.ResponseWriter, r *http.Request) {
	input := r.FormValue("input")
	if input == "" {
		http.Error(w, "input required", http.StatusBadRequest)
		return
	}

	slog.Error("Processing failed: " + input)

	RespondJSON(w, http.StatusInternalServerError, map[string]string{"error": "processing failed"})
}
