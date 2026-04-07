package testcode

import (
	"net/http"
)

func BenchmarkTest01198(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	userID := r.URL.Query().Get("user_id")
	rows, err := DB.Query("SELECT id, name, created_at FROM posts WHERE user_id = ? ORDER BY created_at DESC LIMIT 20", userID)
	if err != nil {
		http.Error(w, "query failed", http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	var posts []map[string]string
	for rows.Next() {
		var id, name, createdAt string
		rows.Scan(&id, &name, &createdAt)
		posts = append(posts, map[string]string{"id": id, "name": name, "created_at": createdAt})
	}
	RespondJSON(w, http.StatusOK, posts)
}
