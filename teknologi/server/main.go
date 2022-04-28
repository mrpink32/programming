package main

import (
	"bufio"
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
		message, err := bufio.NewReader(netStream).ReadString('\n')
		if err != nil {
			netStream.Close()
			break
		}
		fmt.Print("Message Received: ", string(message))
	}
}
