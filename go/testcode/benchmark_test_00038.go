package testcode

import (
	"context"
	"fmt"
	"net/http"
)

func BenchmarkTest00038(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	ctx := context.WithValue(r.Context(), "user_id", id)
	benchmarkTest00038Process(ctx, w)
}

func benchmarkTest00038Process(ctx context.Context, w http.ResponseWriter) {
	userID := ctx.Value("user_id").(string)
	query := fmt.Sprintf("SELECT * FROM users WHERE id = %s", userID)
	rows, err := DB.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
