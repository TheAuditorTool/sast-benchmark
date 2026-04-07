package testcode

import (
	"context"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

var benchmarkTest01022Patterns = map[string]primitive.Regex{
	"starts_with_a": {Pattern: "^a", Options: "i"},
	"contains_test": {Pattern: "test", Options: "i"},
	"ends_with_com": {Pattern: "\\.com$", Options: "i"},
}

func BenchmarkTest01022(w http.ResponseWriter, r *http.Request) {
	patternKey := r.URL.Query().Get("pattern")
	regex, ok := benchmarkTest01022Patterns[patternKey]
	if !ok {
		http.Error(w, "invalid pattern", http.StatusBadRequest)
		return
	}
	filter := bson.M{"email": regex}
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
