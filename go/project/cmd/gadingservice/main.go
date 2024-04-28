package main

import (
	"context"
	"distributed/grades"
	"distributed/registry"
	"distributed/service"
	"fmt"
	stlog "log"
)

func main() {
	host, port := "localhost", "4000"
	serviceAddress := fmt.Sprintf("http://%s:%s", host, port)

	r := registry.Registration{
		ServiceName: registry.GadingService,
		ServiceURL:  serviceAddress,
	}

	ctx, err := service.Start(context.Background(), host, port, r, grades.RegisterHandlers)

	if err != nil {
		stlog.Fatalf("Failed to start server: %v", err)
	}
	<-ctx.Done()
	fmt.Println("Shutting down Gading Service")
}
