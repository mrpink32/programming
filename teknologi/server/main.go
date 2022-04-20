package main

import (
	"fmt"
	"net"
)

func main() {
	fmt.Println("Starting server...")
	// open server socket
	serverSocket, _ := net.Listen("tcp", ":9000")
	fmt.Println("Server started!")
	fmt.Println("Waitinng for client...")
	// accept connection
	netStream, _ := serverSocket.Accept()
	fmt.Println("Client connected!")
	// infinite loop
	for {
		fmt.Fprintf(netStream, "50\n")
	}
}
