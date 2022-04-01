package main

import (
	"bufio"
	"fmt"
	nw "main/networking"
	"net"
)

func main() {
	fmt.Println("Hello World!")
	nw.Test()
	fmt.Println("Starting server...")
	// listen on port 9000
	serverSocket, err := net.Listen("tcp", ":9000")
	println(err)
	fmt.Println("Server started!")
	// accept connection
	netStream, _ := serverSocket.Accept()
	fmt.Println("Client connected!")
	// infinite while loop
	for {
		fmt.Println("Waiting for message...")
		message, _ := bufio.NewReader(netStream).ReadString('\n')
		fmt.Print("Message Received: ", string(message))
		fmt.Fprintf(netStream, "Echo: "+message+"\n")
	}
}
