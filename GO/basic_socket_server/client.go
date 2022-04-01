package main

import (
	"bufio"
	"fmt"
	"net"
	"os"
)

func main() {
	netStream, _ := net.Dial("tcp", "127.0.0.1:9000")
	for {
		// send message from console input
		reader := bufio.NewReader(os.Stdin)
		fmt.Print("Text to send: ")
		text, _ := reader.ReadString('\n')
		fmt.Fprintf(netStream, text+"\n")
		// receive message from server
		message, _ := bufio.NewReader(netStream).ReadString('\n')
		fmt.Print("Message from server: " + message)
	}
}
