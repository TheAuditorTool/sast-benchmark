package testcode

import (
	"context"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

type benchmarkTest01001FilterKey struct{}

func BenchmarkTest01001(w http.ResponseWriter, r *http.Request) {
	field := r.URL.Query().Get("field")
	value := r.URL.Query().Get("value")
	filter := bson.M{field: value}
	ctx := context.WithValue(r.Context(), benchmarkTest01001FilterKey{}, filter)
	f := ctx.Value(benchmarkTest01001FilterKey{}).(bson.M)
	cursor, err := MongoCollection.Find(context.Background(), f)
	if err != nil {
		http.Error(w, "query error", http.StatusInternalServerError)
		return
	}
	defer cursor.Close(context.Background())
	var results []bson.M
	cursor.All(context.Background(), &results)
	RespondJSON(w, http.StatusOK, results)
}
