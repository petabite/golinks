package server

import (
	"embed"
	"os"
	"strconv"

	"github.com/gin-contrib/cors"
	"github.com/gin-contrib/static"
	"github.com/gin-gonic/gin"
	"github.com/petabite/golinks/internal/controllers"
	"github.com/petabite/golinks/internal/database"
)

type Server struct {
	engine     *gin.Engine
	controller *controllers.Controller
}

//go:embed static/*
var clientStatic embed.FS

func Start(port int, dbPath string) {
	gin.SetMode(gin.ReleaseMode)
	env := os.Getenv("ENV")
	if env == "dev" {
		gin.SetMode(gin.DebugMode)
	}

	db := database.NewConn(dbPath)
	controller := controllers.NewController(db)

	r := Server{engine: gin.Default(), controller: controller}
	r.engine.Use(static.Serve("/", EmbedFolder(clientStatic, "static")))
	r.engine.Use(cors.New(cors.Config{
		AllowOrigins:     []string{"*"},
		AllowMethods:     []string{"GET", "PUT", "DELETE", "POST"},
		AllowHeaders:     []string{"Origin"},
		ExposeHeaders:    []string{"Content-Length"},
		AllowCredentials: true,
	}))
	r.bindRoutes()
	r.engine.Run(":" + strconv.Itoa(port))
}
