package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"runtime"
	"strings"
)

func scanString(s *bufio.Scanner) (string, error) {
	if s.Scan() {
		return s.Text(), nil
	}
	err := s.Err()
	if err == nil {
		err = io.EOF
	}
	return "", err
}

func linuxMain() {
	for {
		fmt.Print("> ")
		var input string
		input, err := scanString(bufio.NewScanner(os.Stdin))
		if err != nil {
			fmt.Println(err)
		}

		var parts []string = strings.Split(input, " ")
		var command string = parts[0]
		var args []string

		for i := 1; i < len(parts); i++ {
			args = append(args, parts[i])
		}
		switch command {
		case "ls":
			files, err := os.ReadDir(".")
			if err != nil {
				log.Fatal(err)
			}

			for _, file := range files {
				fmt.Println(file.Name())
			}
			break
		case "exit":
			return
		default:
			break
		}
	}
}

func main() {
	switch runtime.GOOS {
	case "linux":
		linuxMain()
		break
	default:
		break
	}
	// var kernel32 = syscall.NewLazyDLL("kernel32.dll")
	// var user32 = syscall.NewLazyDLL("user32.dll")
}
