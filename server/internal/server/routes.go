package server

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/petabite/golinks/internal/models"
)

func (s *Server) bindRoutes() {
	// Resolve GoLink
	s.engine.GET("/:name", func(c *gin.Context) {
		name := c.Param("name")
		golink := s.controller.GetGoLink(name)
		c.Redirect(http.StatusMovedPermanently, golink.Target)
	})

	// Get all GoLinks
	s.engine.GET("/link", func(c *gin.Context) {
		golinks := s.controller.GetAllGoLinks()
		c.JSON(http.StatusOK, golinks)
	})

	// Create GoLink
	s.engine.POST("/link", func(c *gin.Context) {
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
