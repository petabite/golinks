package server

import (
	"embed"
	"io/fs"
	"net/http"

	"github.com/gin-contrib/static"
)

// adapts embed.FS to static.ServeFileSystem
// credit: https://github.com/gin-contrib/static/issues/19
type embedFileSystem struct {
	http.FileSystem
}

func (e embedFileSystem) Exists(prefix string, path string) bool {
	_, err := e.Open(path)
	return err == nil
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
