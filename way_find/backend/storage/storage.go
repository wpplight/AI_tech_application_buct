package storage

import (
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"time"

	mappkg "wayfind/backend/map"
)

var (
	ErrMapNotFound    = errors.New("地图不存在")
	ErrInvalidMapName = errors.New("无效的地图名称")
	ErrStorageNotSet  = errors.New("存储路径未设置")
)

type MapInfo struct {
	Name        string    `json:"name"`
	Width       int       `json:"width"`
	Height      int       `json:"height"`
	CreatedAt   time.Time `json:"createdAt"`
	ModifiedAt  time.Time `json:"modifiedAt"`
}

type MapStorage interface {
	Save(name string, m *mappkg.Map) error
	Load(name string) (*mappkg.Map, error)
	Delete(name string) error
	List() ([]MapInfo, error)
	GetStoragePath() string
	SetStoragePath(path string) error
}

type FileMapStorage struct {
	basePath string
}

func NewFileMapStorage(basePath string) (*FileMapStorage, error) {
	if basePath == "" {
		return nil, ErrStorageNotSet
	}

	if err := os.MkdirAll(basePath, 0755); err != nil {
		return nil, err
	}

	return &FileMapStorage{basePath: basePath}, nil
}

func (s *FileMapStorage) GetStoragePath() string {
	return s.basePath
}

func (s *FileMapStorage) SetStoragePath(path string) error {
	if path == "" {
		return ErrStorageNotSet
	}
	if err := os.MkdirAll(path, 0755); err != nil {
		return err
	}
	s.basePath = path
	return nil
}

func (s *FileMapStorage) Save(name string, m *mappkg.Map) error {
	if name == "" {
		return ErrInvalidMapName
	}

	safeName := sanitizeFileName(name)
	filePath := s.getMapFilePath(safeName)
	infoPath := s.getInfoFilePath(safeName)

	text, err := m.ToText()
	if err != nil {
		return err
	}

	if err := os.WriteFile(filePath, []byte(text), 0644); err != nil {
		return err
	}

	info := MapInfo{
		Name:       name,
		Width:      m.Width,
		Height:     m.Height,
		CreatedAt:  time.Now(),
		ModifiedAt: time.Now(),
	}

	infoData, err := json.MarshalIndent(info, "", "  ")
	if err != nil {
		return err
	}

	return os.WriteFile(infoPath, infoData, 0644)
}

func (s *FileMapStorage) Load(name string) (*mappkg.Map, error) {
	if name == "" {
		return nil, ErrInvalidMapName
	}

	safeName := sanitizeFileName(name)
	filePath := s.getMapFilePath(safeName)

	content, err := os.ReadFile(filePath)
	if err != nil {
		if os.IsNotExist(err) {
			return nil, ErrMapNotFound
		}
		return nil, err
	}

	return mappkg.MapFromText(string(content))
}

func (s *FileMapStorage) Delete(name string) error {
	if name == "" {
		return ErrInvalidMapName
	}

	safeName := sanitizeFileName(name)
	mapPath := s.getMapFilePath(safeName)
	infoPath := s.getInfoFilePath(safeName)

	var errs []error

	if err := os.Remove(mapPath); err != nil && !os.IsNotExist(err) {
		errs = append(errs, err)
	}

	if err := os.Remove(infoPath); err != nil && !os.IsNotExist(err) {
		errs = append(errs, err)
	}

	if len(errs) > 0 {
		return errs[0]
	}
	return nil
}

func (s *FileMapStorage) List() ([]MapInfo, error) {
	entries, err := os.ReadDir(s.basePath)
	if err != nil {
		return nil, err
	}

	var maps []MapInfo
	for _, entry := range entries {
		if entry.IsDir() {
			continue
		}

		baseName := entry.Name()
		if filepath.Ext(baseName) != ".info" {
			continue
		}

		infoPath := filepath.Join(s.basePath, baseName)
		data, err := os.ReadFile(infoPath)
		if err != nil {
			continue
		}

		var info MapInfo
		if err := json.Unmarshal(data, &info); err != nil {
			continue
		}

		maps = append(maps, info)
	}

	return maps, nil
}

func (s *FileMapStorage) getMapFilePath(name string) string {
	return filepath.Join(s.basePath, name+".map")
}

func (s *FileMapStorage) getInfoFilePath(name string) string {
	return filepath.Join(s.basePath, name+".info")
}

func sanitizeFileName(name string) string {
	name = filepath.Base(name)
	if name == "." || name == "" {
		return "unnamed"
	}
	return name
}
