package leg

import (
	"fmt"
	"os"

	"github.com/fatih/color"
)

// Creates an error for required parameters.
func newErrorRequiredParam(name string) error {
	return fmt.Errorf("missing required parameter '%s'", name)
}

// Prints a line with custom format.

// - `tag`: String to use as prefix (after scope).
// - `message`: String to print.
// - `scope`: Prefix to append.
func printLine(
	tag string,
	col color.Attribute,
	message string,
	scope string,
) error {
	if tag == "" {
		return newErrorRequiredParam("tag")
	}
	if col == 0 {
		return newErrorRequiredParam("col")
	}
	if message == "" {
		return newErrorRequiredParam("message")
	}
	if scope != "" {
		scope = fmt.Sprintf("[%s]", scope)
	}
	tagWithColor := color.New(col, color.Bold).SprintFunc()(tag)
	line := fmt.Sprintf("%s %s %s\n", scope, tagWithColor, message)
	_, err := os.Stderr.WriteString(line)
	return err
}

// Head prints a simple application header/title.
//
// - `name`: Name of the project.
// - `icon`: Optional icon to display.
// - `version`: Include also the version.
func Head(name string, icon string, version string) error {
	if version != "" {
		version = fmt.Sprintf("\t(v%s)", version)
	}
	_, err := os.Stderr.WriteString(fmt.Sprintf(
		"\n\t%s %s\n%s\n\n",
		icon,
		color.New(color.Bold).SprintFunc()(name),
		color.New(color.Faint).SprintFunc()(version),
	))
	return err
}

// Info prints generic data.
//
// - `message`: String to print.
// - `scope`: Prefix to append.
func Info(message string, scope string) error {
	return printLine("ℹ", color.FgHiBlue, message, scope)
}

// Success prints about a correct operation.
//
// - `message`: String to print.
// - `scope`: Prefix to append.
func Success(message string, scope string) error {
	return printLine("✔", color.FgHiGreen, message, scope)
}

// Warn prints about a not completely correct operation.
//
// - `message`: String to print.
// - `scope`: Prefix to append.
func Warn(message string, scope string) error {
	return printLine("⚠", color.FgHiYellow, message, scope)
}

// Error prints about an errored operation.
//
// - `message`: String to print.
// - `scope`: Prefix to append.
func Error(message string, scope string) error {
	return printLine("✖", color.FgHiRed, message, scope)
}

// Wait indicates a delay.
//
// - `message`: String to print.
// - `scope`: Prefix to append.
func Wait(message string, scope string) error {
	return printLine("…", color.FgHiMagenta, message, scope)
}

// Done indicates something finished.
//
// - `message`: String to print.
// - `scope`: Prefix to append.
func Done(message string, scope string) error {
	return printLine("☒", color.FgHiCyan, message, scope)
}

// Remove deletes the actual line..
func Remove() error {
	_, err := os.Stderr.WriteString("\r")
	return err
}

// Result prints to the standard output.
//
// - `message`: String to print.
func Result(message string) error {
	if message == "" {
		return newErrorRequiredParam("message")
	}
	_, err := os.Stdout.WriteString(message + "\n")
	return err
}
