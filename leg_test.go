package leg

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestHead(t *testing.T) {
	err := Head("mumu", "🐘", "0.1.0")
	assert.NoError(t, err)
}

func TestInfo(t *testing.T) {
	err := Info("Informational message", "")
	assert.NoError(t, err)

	err = Info("Informational message with scope", "scope-0")
	assert.NoError(t, err)
}

func TestSuccess(t *testing.T) {
	err := Success("Successful operation", "")
	assert.NoError(t, err)
}

func TestWarn(t *testing.T) {
	err := Warn("Warn message", "")
	assert.NoError(t, err)
}

func TestError(t *testing.T) {
	err := Error("Error message", "")
	assert.NoError(t, err)
}

func TestWait(t *testing.T) {
	err := Wait("Waiting for something", "")
	assert.NoError(t, err)
}

func TestDone(t *testing.T) {
	err := Done("Something finished", "")
	assert.NoError(t, err)
}

func TestRemove(t *testing.T) {
	_, err := fmt.Print("deleted line")
	assert.NoError(t, err)
	err = Remove()
	assert.NoError(t, err)
}

func TestResult(t *testing.T) {
	err := Result("To standard output")
	assert.NoError(t, err)
}
