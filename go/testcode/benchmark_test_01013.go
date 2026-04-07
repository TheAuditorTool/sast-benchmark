package testcode

import (
	"context"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

func BenchmarkTest01013(w http.ResponseWriter, r *http.Request) {
	userID := r.URL.Query().Get("user_id")
	filter := bson.M{
		"$and": []bson.M{
			{"user_id": userID},
			{"status": "active"},
		},
	}
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
