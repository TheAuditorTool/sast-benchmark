package testcode

import (
	"net/http"

	"go.mongodb.org/mongo-driver/bson"
)

func BenchmarkTest00355(w http.ResponseWriter, r *http.Request) {
	name := r.URL.Query().Get("name")
	filter := bson.M{"username": name}
	cursor, err := MongoCollection.Find(r.Context(), filter)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer cursor.Close(r.Context())
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
