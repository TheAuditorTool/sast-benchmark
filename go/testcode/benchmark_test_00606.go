package testcode

import (
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

func BenchmarkTest00606(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	field := r.FormValue("field")
	value := r.FormValue("value")

	if id == "" || field == "" {
		http.Error(w, "id and field required", http.StatusBadRequest)
		return
	}

	update := bson.M{"$set": bson.M{field: value}}
	result, err := MongoCollection.UpdateOne(r.Context(), bson.M{"_id": id}, update)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"matched":  result.MatchedCount,
		"modified": result.ModifiedCount,
	})
}
