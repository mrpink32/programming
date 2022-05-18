package main

import (
	"bufio"
	"errors"
	"fmt"
	"io/fs"
	"net"
	"os"
)

func errorCheck(err error /*, expected_err error*/) {
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

func main() {
	file, err := os.Open("words.data")
	if err != nil {
		if errors.Is(err, fs.ErrNotExist) {
			os.Create("words.data")
		} else {
			panic(err)
		}
	}

	serverSocket := StartServer("tcp", ":9000")
	var netStream net.Conn
	for {
		fmt.Println("Waitinng for client...")
		// accept connection
		netStream, err = serverSocket.Accept()
		errorCheck(err)
		fmt.Println("Client connected!")
		// infinite while loop
		for {
			fmt.Println("Waiting for message...")
			message, err := bufio.NewReader(netStream).ReadString('\n')
			if err != nil {
				println(err)
				break
			}
			fmt.Print("Message Received: ", message)
			// todo handel message

			switch message[:0] {
			case "0:":
				fmt.Println("Send some data from file!")
			case "1:":
				fmt.Println("Receive new data and add to file!")
				// todo split message and add to file
			default:
				fmt.Println("Default")
			}

		}
	}
	file.Close()
}
