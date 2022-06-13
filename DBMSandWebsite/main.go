package main

import (
	"bufio"
	"errors"
	"fmt"
	"io/fs"
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
	file, err := os.Open("words.data")
	if err != nil {
		if errors.Is(err, fs.ErrNotExist) {
			os.Create("words.data")
		} else {
			panic(err)
		}
	}

	// fmt.Println(file.ReadString('\n'))

	serverSocket := StartServer("tcp", ":9000")
	// var netStream net.Conn
	for {
		fmt.Println("Waitinng for client...")
		// accept connection
		netStream, err := serverSocket.Accept()
		errorCheck(err)
		fmt.Println("Client connected!")
		// infinite while loop
		for {
			fmt.Println("Waiting for message...")
			// var message string = "text works"
			message, err := readFromNetstream(netStream)
			if err != nil {
				println(err)
				netStream.Close()
				break
			}

			// _, err := printToNetstream(netStream, message)
			// if err != nil {
			// 	println(err)
			// 	netStream.Close()
			// 	break
			// }

			// todo handel message

			switch message[:2] {
			case "0:":
				// choose words to get

			case "1:":
				// fmt.Println("Receive new data and add to file!")
				reader := bufio.NewReader(file)
				line, _ := reader.ReadString('\n')
				printToNetstream(netStream, line)
				println(line)
				// todo split message and add to file
			case "2:":
				// check answer
				fmt.Println("check: " + message[2:])
			default:
				fmt.Println(message)
			}

		}
	}
	file.Close()
}
