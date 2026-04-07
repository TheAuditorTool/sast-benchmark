package testcode

import (
	"context"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

func BenchmarkTest00996(w http.ResponseWriter, r *http.Request) {
	userOperator := r.URL.Query().Get("op")
	userValue := r.URL.Query().Get("value")
	filter := bson.M{"username": bson.M{userOperator: userValue}}
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
