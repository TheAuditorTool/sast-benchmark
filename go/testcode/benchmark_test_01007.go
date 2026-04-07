package testcode

import (
	"context"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

func BenchmarkTest01007(w http.ResponseWriter, r *http.Request) {
	pattern := r.URL.Query().Get("pattern")
	filter := bson.M{"name": bson.M{"$regex": pattern, "$options": "i"}}
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
