package main

import (
	"bufio"
	"fmt"
	nw "main/networking"
)

func main() {
	fmt.Println("Hello World!")
	serverSocket := nw.StartServer("tcp", ":9000")
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
