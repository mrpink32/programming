package main

import (
	"bufio"
	"fmt"
	"net"
)

func main() {
	fmt.Println("Hello World!")
	fmt.Println("Starting server...")
	serverSocket, _ := net.Listen("tcp", ":9000")
	fmt.Println("Server started!")
	fmt.Println("Waitinng for client...")
	// accept connection
	netStream, _ := serverSocket.Accept()
	fmt.Println("Client connected!")
	// infinite while loop
	for {
		fmt.Println("Waiting for message...")
		message, _ := bufio.NewReader(netStream).ReadString('\n')
		fmt.Print("Message Received: ", string(message))
		fmt.Fprintf(netStream, "Echo {"+message+"}\n")
	}
}
