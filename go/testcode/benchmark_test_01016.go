package testcode

import (
	"context"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

var benchmarkTest01016AllowedFields = map[string]bool{
	"username": true,
	"email":    true,
	"status":   true,
}

func BenchmarkTest01016(w http.ResponseWriter, r *http.Request) {
	field := r.URL.Query().Get("field")
	value := r.URL.Query().Get("value")
	if !benchmarkTest01016AllowedFields[field] {
		http.Error(w, "invalid field", http.StatusBadRequest)
		return
	}
	cursor, err := MongoCollection.Find(context.Background(), bson.M{field: value})
	if err != nil {
		http.Error(w, "query error", http.StatusInternalServerError)
		return
	}
	defer cursor.Close(context.Background())
	var results []bson.M
	cursor.All(context.Background(), &results)
	RespondJSON(w, http.StatusOK, results)
}
