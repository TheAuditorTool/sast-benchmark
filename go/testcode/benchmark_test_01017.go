package testcode

import (
	"context"
	"net/http"
	"strconv"

	"go.mongodb.org/mongo-driver/bson"
)

func BenchmarkTest01017(w http.ResponseWriter, r *http.Request) {
	idStr := r.URL.Query().Get("user_id")
	userID, err := strconv.Atoi(idStr)
	if err != nil {
		http.Error(w, "invalid user id", http.StatusBadRequest)
		return
	}
	filter := bson.M{"user_id": userID}
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
