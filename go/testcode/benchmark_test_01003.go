package testcode

import (
	"context"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func BenchmarkTest01003(w http.ResponseWriter, r *http.Request) {
	idStr := r.URL.Query().Get("id")
	oid, err := primitive.ObjectIDFromHex(idStr)
	if err != nil {
		http.Error(w, "invalid id", http.StatusBadRequest)
		return
	}
	var userFields bson.M
	if err := ParseJSONBody(r, &userFields); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}
	_, err = MongoCollection.UpdateOne(
		context.Background(),
		bson.M{"_id": oid},
		bson.M{"$set": userFields},
	)
	if err != nil {
		http.Error(w, "update error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
}
