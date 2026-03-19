package main

import (
	"database/sql"
	"fmt"
	"log"
	"net/http"

	_ "github.com/mattn/go-sqlite3"
	"github.com/theauditor/go-benchmark/testcode"
)

func main() {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	testcode.SetDB(db)

	// Each benchmark test is registered as a handler.
	// This allows live testing of individual test cases.
	// The test number maps to the CSV ground truth.

	fmt.Println("Go SAST Benchmark v0.1")
	fmt.Println("256 test cases across 12 CWE categories")
	fmt.Println("Listening on :8080")

	log.Fatal(http.ListenAndServe(":8080", nil))
}
