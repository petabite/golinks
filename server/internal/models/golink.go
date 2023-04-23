package models

import (
	"strings"
	"time"

	uuid "github.com/satori/go.uuid"
	"gorm.io/gorm"
)

type GoLink struct {
	ID        uuid.UUID `gorm:"type:uuid;primary_key;" json:"id"`
	CreatedAt time.Time `json:"createdAt"`
	UpdatedAt time.Time `json:"updatedAt"`
	Name      string    `json:"name" gorm:"unique;not null;"`
	Target    string    `json:"target" gorm:"not null;"`
	Visits    int       `json:"visits" gorm:"default:0;"`
}

func (g *GoLink) BeforeCreate(tx *gorm.DB) (err error) {
	g.ID = uuid.NewV4()
	return
}

func (g *GoLink) BeforeSave(tx *gorm.DB) (err error) {
	if !strings.HasPrefix(g.Target, "http") {
		g.Target = "https://" + g.Target
	}
	return
}
