package server

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

type RequestBody struct {
	Name   string `json:"name"`
	Target string `json:"target"`
}

func (s *Server) bindRoutes() {
	// Serve client
	s.engine.GET("/links", func(c *gin.Context) {
		c.FileFromFS("./static/", http.FS(clientStatic))
	})

	// Resolve GoLink
	s.engine.GET("/:name", func(c *gin.Context) {
		name := c.Param("name")
		golink, err := s.controller.GetGoLink(name)
		var redirectLocation string
		if err != nil {
			redirectLocation = "/links?name=" + name
		} else {
			redirectLocation = golink.Target
			err := s.controller.IncrementGoLinkVisit(name)
			if err != nil {
				c.JSON(http.StatusInternalServerError, err.Error())
				return
			}
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
		var request RequestBody
		if err := c.BindJSON(&request); err != nil {
			c.JSON(http.StatusBadRequest, err.Error())
			return
		}

		if request.Name == "" || request.Target == "" {
			c.JSON(http.StatusBadRequest, "'name' and 'target' are required")
			return
		}

		newGoLink, err := s.controller.CreateGoLink(request.Name, request.Target)
		if err != nil {
			c.JSON(http.StatusBadRequest, err.Error())
			return
		}

		c.JSON(http.StatusOK, newGoLink)
	})

	// Update GoLink
	s.engine.PUT("/link/:name", func(c *gin.Context) {
		var request RequestBody
		if err := c.BindJSON(&request); err != nil {
			c.JSON(http.StatusBadRequest, err.Error())
			return
		}
		name := c.Param("name")
		updatedGoLink, err := s.controller.UpdateGoLink(name, request.Target)
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
