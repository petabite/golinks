package controllers

import (
	"errors"
	"fmt"

	"github.com/petabite/golinks/internal/models"
	"gorm.io/gorm"
)

type Controller struct {
	db *gorm.DB
}

func NewController(db *gorm.DB) *Controller {
	return &Controller{db: db}
}

func (c *Controller) CreateGoLink(name, target string) (*models.GoLink, error) {
	newGoLink := &models.GoLink{Name: name, Target: target}
	result := c.db.Create(&newGoLink)

	if result.Error != nil {
		return nil, result.Error
	}

	return newGoLink, nil
}

func (c *Controller) GetAllGoLinks() []models.GoLink {
	var golinks []models.GoLink
	c.db.Find(&golinks)
	return golinks
}

func (c *Controller) GetGoLink(name string) *models.GoLink {
	var golink *models.GoLink
	tx := c.db.Where("name = ?", name).First(&golink)
	if errors.Is(tx.Error, gorm.ErrRecordNotFound) {
		return nil
	}
	return golink
}

func (c *Controller) UpdateGoLink(name, target string) (*models.GoLink, error) {
	golink := c.GetGoLink(name)
	if golink == nil {
		return nil, fmt.Errorf("GoLink with name '%s' does not exist", name)
	}
	golink.Target = target
	result := c.db.Save(golink)
	if result.Error != nil {
		return nil, result.Error
	}
	return golink, nil
}

func (c *Controller) DeleteGoLink(name string) error {
	golink := c.GetGoLink(name)
	if golink == nil {
		return fmt.Errorf("GoLink with name '%s' does not exist", name)
	}
	c.db.Delete(golink)
	return nil
}
