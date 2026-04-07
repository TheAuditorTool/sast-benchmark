package testcode

import (
	"context"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo/options"
)

var benchmarkTest01018SortFields = map[string]bool{
	"created_at": true,
	"name":       true,
	"price":      true,
}

func BenchmarkTest01018(w http.ResponseWriter, r *http.Request) {
	sortField := r.URL.Query().Get("sort")
	if !benchmarkTest01018SortFields[sortField] {
		sortField = "created_at"
	}
	opts := options.Find().SetSort(bson.D{{Key: sortField, Value: 1}})
	cursor, err := MongoCollection.Find(context.Background(), bson.M{"active": true}, opts)
	if err != nil {
		http.Error(w, "query error", http.StatusInternalServerError)
		return
	}
	defer cursor.Close(context.Background())
	var results []bson.M
	cursor.All(context.Background(), &results)
	RespondJSON(w, http.StatusOK, results)
}
