package testcode

import (
	"context"
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func BenchmarkTest01020(w http.ResponseWriter, r *http.Request) {
	idStr := r.URL.Query().Get("id")
	oid, err := primitive.ObjectIDFromHex(idStr)
	if err != nil {
		http.Error(w, "invalid id", http.StatusBadRequest)
		return
	}
	newName := r.FormValue("name")
	_, err = MongoCollection.UpdateOne(
		context.Background(),
		bson.M{"_id": oid},
		bson.M{"$set": bson.M{"name": newName}},
	)
	if err != nil {
		http.Error(w, "update error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
}
