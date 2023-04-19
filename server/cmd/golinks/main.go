package main

import (
	"flag"

	"github.com/petabite/golinks/internal/server"
)

func main() {
	port := flag.Int("port", 7890, "port to run server on")
	dbPath := flag.String("db", "golinks.sqlite", "path to database file")
	flag.Parse()
	
	server.Start(*port, *dbPath)
}
