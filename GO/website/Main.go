package main

import (
	"bufio"
	"fmt"
	"net"
	"os"
	"strings"
)

func errorCheck(err error) {
	if err != nil {
		panic(err)
	}
}

func StartServer(network string, port string) net.Listener {
	fmt.Println("Starting server...")
	serverSocket, err := net.Listen(network, port)
	errorCheck(err)
	fmt.Println("Server started!")
	return serverSocket
}

func ServerHandler(serverSocket net.Listener) {

}

func printToNetstream(netStream net.Conn, message string) (int, error) {
	/// Print to netstream ///
	// wrote, err := bufio.NewWriter(netStream).WriteString(message + "\n")
	wrote, err := fmt.Fprint(netStream, message+"\n")
	println("Message Sent: ", message)
	return wrote, err
}

func readFromNetstream(netStream net.Conn) (string, error) {
	/// read from netstream ///
	message, err := bufio.NewReader(netStream).ReadString('\n')
	return strings.ReplaceAll(message, "\n", ""), err
}

func main() {
	serverSocket := StartServer("tcp", ":9000")
	go ServerHandler(serverSocket)
	for {
		var input string
		fmt.Scanln(input)
		if input != "" {
			fmt.Println(input)
			switch input {
			case "Shutdown":
				os.Exit(0)
			default:
				fmt.Println("Command not recognised")
			}
		}
	}
}
