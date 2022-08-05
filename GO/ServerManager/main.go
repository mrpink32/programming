package main

import "syscall"

func main() {
	var kernel32 = syscall.NewLazyDLL("kernel32.dll")
	var user32 = syscall.NewLazyDLL("user32.dll")

}
