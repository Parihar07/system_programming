package main

import (
	"log"
	"net/http"
	"ws/internal/handlers"
)

func main() {
	println("hello parihar")
	mux := routes()

	log.Println("starting channel listner")
	go handlers.ListenToWsChannel()
	
	log.Println("starting web server on port 8080")
 
	_ = http.ListenAndServe(":8080", mux)

}

