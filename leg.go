// Package leg provides helpers to print the most common type of messages
// used in command line applications.
//
// For simplicity, parameters of the different functions are not validated.
// It is not fun to check for errors when printing messages. The benefits would
// not be worth it.
package leg

import (
	"fmt"
	"os"

	"github.com/fatih/color"
)

// Head prints a simple application header/title.
//
// - `name`: Name of the project.
// - `icon`: Optional icon to display.
// - `version`: Include also the version.
func Head(name string, icon string, version string) {
	if version != "" {
		version = fmt.Sprintf("\t(v%s)", version)
	}
	fmt.Fprintf(
		os.Stderr,
		"\n\t%s %s\n%s\n\n",
		icon,
		color.New(color.Bold).SprintFunc()(name),
		color.New(color.Faint).SprintFunc()(version),
	)
}

// Info prints generic data.
//
// - `message`: String to print.
// - `scope`: Prefix to append.
func Info(message string, scope string) {
	printLine(&params{
		tag:     "ℹ",
		color:   color.FgHiBlue,
		message: message,
		scope:   scope,
	})
}

// Prints a line with custom format to stderr.
func printLine(p *params) {
	if p.scope != "" {
		p.scope = fmt.Sprintf("[%s]", p.scope)
	}
	tagWithColor := color.New(p.color, color.Bold).SprintFunc()(p.tag)
	fmt.Fprintf(os.Stderr, "%s %s %s\n", p.scope, tagWithColor, p.message)
}

// Parameters for `printLine` function.
type params struct {
	// String to use as prefix (after scope).
	tag string
	// Color to use for the tag.
	color color.Attribute
	// String to print.
	message string
	// Prefix to append, optional.
	scope string
}

// Success prints about a correct operation.
//
// - `message`: String to print.
// - `scope`: Prefix to append.
func Success(message string, scope string) {
	printLine(&params{
		tag:     "✔",
		color:   color.FgHiGreen,
		message: message,
		scope:   scope,
	})
}

// Warn prints about a not completely correct operation.
//
// - `message`: String to print.
// - `scope`: Prefix to append.
func Warn(message string, scope string) {
	printLine(&params{
		tag:     "⚠",
		color:   color.FgHiYellow,
		message: message,
		scope:   scope,
	})
}

// Error prints about an errored operation.
//
// - `message`: String to print.
// - `scope`: Prefix to append.
func Error(message string, scope string) {
	printLine(&params{
		tag:     "✖",
		color:   color.FgHiRed,
		message: message,
		scope:   scope,
	})
}

// Wait indicates a delay.
//
// - `message`: String to print.
// - `scope`: Prefix to append.
func Wait(message string, scope string) {
	printLine(&params{
		tag:     "…",
		color:   color.FgHiMagenta,
		message: message,
		scope:   scope,
	})
}

// Done indicates something finished.
//
// - `message`: String to print.
// - `scope`: Prefix to append.
func Done(message string, scope string) {
	printLine(&params{
		tag:     "☒",
		color:   color.FgHiCyan,
		message: message,
		scope:   scope,
	})
}

// Remove deletes the actual line..
func Remove() error {
	_, err := os.Stderr.WriteString("\r")
	return err
}

// Result prints to the standard output.
//
// - `message`: String to print.
func Result(message string) {
	fmt.Fprintln(os.Stderr, message+"\n")
}
