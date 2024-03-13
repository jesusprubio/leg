// Usage: go run examples/readme.go
package main

import (
	"fmt"

	"github.com/jesusprubio/leg"
)

func main() {
	leg.Head("mumu", "🐘", "0.1.0")
	leg.Info("Informational message", "")
	leg.Success("Successful operation", "")
	leg.Warn("Warn message", "")
	leg.Error("Error message", "")
	leg.Wait("Waiting for something", "")
	leg.Done("Something finished", "")

	leg.Info("Informational message with scope", "scope-0")

	fmt.Print("deleted line")
	leg.Remove()

	leg.Result("To standard output")
}
