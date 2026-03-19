package testcode

import (
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
)

var MongoCollection *mongo.Collection

func BenchmarkTest00351(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("search")
	filter := bson.M{"$where": "this.name == '" + input + "'"}
	cursor, err := MongoCollection.Find(r.Context(), filter)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer cursor.Close(r.Context())
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
