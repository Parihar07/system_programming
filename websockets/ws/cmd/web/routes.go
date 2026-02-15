package main

import (
	"net/http"
	// Update the import path below to the correct location of your handlers package.
	// For example, if handlers.go is in the same directory as routes.go, use:
		"ws/internal/handlers"
	// Or, if it's in a different relative path, adjust accordingly, e.g.:
	//	"../internal/handlers"

	"github.com/bmizerany/pat"
)

func routes() http.Handler {
	mux := pat.New()
	mux.Get("/", http.HandlerFunc(handlers.Home))
	mux.Get("/ws", http.HandlerFunc(handlers.WsEndPoint))
	return mux
}