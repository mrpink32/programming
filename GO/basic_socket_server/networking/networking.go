package networking

import (
	"fmt"
	"net"
)

func StartServer(network string, port string) net.Listener {
	fmt.Println("Starting server...")
	serverSocket, _ := net.Listen(network, port)
	fmt.Println("Server started!")
	return serverSocket
}
