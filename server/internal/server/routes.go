package server

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/petabite/golinks/internal/models"
)

func (s *Server) bindRoutes() {
	// Serve client
	s.engine.GET("/links", func(c *gin.Context) {
		c.FileFromFS("./static/index.html", http.FS(clientStatic))
	})

	// Resolve GoLink
	s.engine.GET("/:name", func(c *gin.Context) {
		name := c.Param("name")
		golink := s.controller.GetGoLink(name)
		var redirectLocation string
		if golink == nil {
			redirectLocation = "/links" // TODO: query string to autofill name in ui
		} else {
			redirectLocation = golink.Target
		}
		c.Redirect(http.StatusMovedPermanently, redirectLocation)
	})

	// Get all GoLinks
	s.engine.GET("/link", func(c *gin.Context) {
		golinks := s.controller.GetAllGoLinks()
		c.JSON(http.StatusOK, golinks)
	})

	// Create GoLink
	s.engine.POST("/link", func(c *gin.Context) {
		// TODO: make new struct for request body
		var golink models.GoLink
		if err := c.BindJSON(&golink); err != nil {
			c.JSON(http.StatusBadRequest, err.Error())
			return
		}

		if golink.Name == "" || golink.Target == "" {
			c.JSON(http.StatusBadRequest, "'name' and 'target' are required")
			return
		}

		newGoLink, err := s.controller.CreateGoLink(golink.Name, golink.Target)
		if err != nil {
			c.JSON(http.StatusBadRequest, err.Error())
			return
		}

		c.JSON(http.StatusOK, newGoLink)
	})

	// Update GoLink
	s.engine.PUT("/link/:name", func(c *gin.Context) {
		var golink models.GoLink
		if err := c.BindJSON(&golink); err != nil {
			c.JSON(http.StatusBadRequest, err.Error())
			return
		}
		name := c.Param("name")
		updatedGoLink, err := s.controller.UpdateGoLink(name, golink.Target)
		if err != nil {
			c.JSON(http.StatusBadRequest, err.Error())
			return
		}
		c.JSON(http.StatusOK, updatedGoLink)
	})

	// Delete GoLink
	s.engine.DELETE("/link/:name", func(c *gin.Context) {
		name := c.Param("name")
		err := s.controller.DeleteGoLink(name)
		if err != nil {
			c.JSON(http.StatusBadRequest, err.Error())
			return
		}
		c.JSON(http.StatusOK, "Deleted golink!")
	})
}
