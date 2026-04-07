package testcode

import (
	"context"
	"encoding/json"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

func BenchmarkTest01005(w http.ResponseWriter, r *http.Request) {
	userID := r.URL.Query().Get("user_id")
	var prefJSON string
	err := DB.QueryRow("SELECT filter_pref FROM user_settings WHERE user_id = ?", userID).Scan(&prefJSON)
	if err != nil {
		http.Error(w, "settings not found", http.StatusNotFound)
		return
	}
	var filter bson.M
	json.Unmarshal([]byte(prefJSON), &filter)
	cursor, err := MongoCollection.Find(context.Background(), filter)
	if err != nil {
		http.Error(w, "query error", http.StatusInternalServerError)
		return
	}
	defer cursor.Close(context.Background())
	var results []bson.M
	cursor.All(context.Background(), &results)
	RespondJSON(w, http.StatusOK, results)
}
