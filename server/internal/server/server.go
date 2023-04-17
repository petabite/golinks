package server

import (
	"net/http"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	"github.com/petabite/golinks/internal/controllers"
	"github.com/petabite/golinks/internal/database"
)

type Server struct {
	engine     *gin.Engine
	controller *controllers.Controller
}

func ErrorHandler(c *gin.Context) {
	err := c.Errors.Last() // Get the last error
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err})
	}
}

func Start() {
	db := database.NewConn()
	controller := controllers.NewController(db)

	r := Server{engine: gin.Default(), controller: controller}
	r.engine.Use(cors.New(cors.Config{
		AllowOrigins:     []string{"*"},
		AllowMethods:     []string{"PUT", "PATCH", "POST"},
		AllowHeaders:     []string{"Origin"},
		ExposeHeaders:    []string{"Content-Length"},
		AllowCredentials: true,
	}))
	r.bindRoutes()
	r.engine.Run(":9888")
}
