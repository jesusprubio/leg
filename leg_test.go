package leg

import (
	"fmt"
	"testing"
)

func TestHead(t *testing.T) {
	Head("mumu", "🐘", "0.1.0")
}

func TestInfo(t *testing.T) {
	Info("Informational message", "")
	Info("Informational message with scope", "scope-0")
}

func TestSuccess(t *testing.T) {
	Success("Successful operation", "")
}

func TestWarn(t *testing.T) {
	Warn("Warn message", "")
}

func TestError(t *testing.T) {
	Error("Error message", "")
}

func TestWait(t *testing.T) {
	Wait("Waiting for something", "")
}

func TestDone(t *testing.T) {
	Done("Something finished", "")
}

func TestRemove(t *testing.T) {
	fmt.Print("deleted line")
	Remove()
}

func TestResult(t *testing.T) {
	Result("To standard output")
}
