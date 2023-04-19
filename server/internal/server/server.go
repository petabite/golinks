package server

import (
	"embed"
	"io/fs"
	"net/http"
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

func ErrorHandler(c *gin.Context) {
	err := c.Errors.Last() // Get the last error
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err})
	}
}

// TODO: extract
// credit: https://github.com/gin-contrib/static/issues/19
type embedFileSystem struct {
	http.FileSystem
}

func (e embedFileSystem) Exists(prefix string, path string) bool {
	_, err := e.Open(path)
	if err != nil {
		return false
	}
	return true
}

func EmbedFolder(fsEmbed embed.FS, targetPath string) static.ServeFileSystem {
	fsys, err := fs.Sub(fsEmbed, targetPath)
	if err != nil {
		panic(err)
	}
	return embedFileSystem{
		FileSystem: http.FS(fsys),
	}
}

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
