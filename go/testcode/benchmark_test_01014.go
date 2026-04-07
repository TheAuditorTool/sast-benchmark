package testcode

import (
	"context"
	"net/http"
	"net/mail"

	"go.mongodb.org/mongo-driver/bson"
)

func BenchmarkTest01014(w http.ResponseWriter, r *http.Request) {
	email := r.URL.Query().Get("email")
	if _, err := mail.ParseAddress(email); err != nil {
		http.Error(w, "invalid email", http.StatusBadRequest)
		return
	}
	filter := bson.M{"email": email}
	var result bson.M
	err := MongoCollection.FindOne(context.Background(), filter).Decode(&result)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	RespondJSON(w, http.StatusOK, result)
}
