package server

import (
	"net/http"

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
	r.engine.Use(ErrorHandler)
	r.bindRoutes()
	r.engine.Run()
}
