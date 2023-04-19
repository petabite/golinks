package database

import (
	"github.com/petabite/golinks/internal/models"
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

func NewConn(dbPath string) *gorm.DB {
	db, err := gorm.Open(sqlite.Open(dbPath), &gorm.Config{})
	if err != nil {
		panic("failed to connect database")
	}

	// Migrations
	db.AutoMigrate(&models.GoLink{})

	return db
}
