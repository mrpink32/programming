package main

import (
	"fmt"
	"net"
	"time"
)

func main() {
	fmt.Println("Starting server...")
	// open server socket
	serverSocket, _ := net.Listen("tcp", ":9000")
	fmt.Println("Server started!")
	for {
		fmt.Println("Waitinng for client...")
		// accept connection
		netStream, _ := serverSocket.Accept()
		fmt.Println("Client connected!")
		// infinite loop
		for {
			wrote, err := fmt.Fprint(netStream, rune(2075))
			if err != nil {
				netStream.Close()
				break
			}
			println(wrote)
			time.Sleep(5 * time.Second)
		}
	}
}
