package testcode

import (
	"context"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

func benchmarkTest00998BuildFilter(field, value string) bson.M {
	return bson.M{field: value}
}

func BenchmarkTest00998(w http.ResponseWriter, r *http.Request) {
	filter := benchmarkTest00998BuildFilter(r.URL.Query().Get("field"), r.URL.Query().Get("value"))
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
