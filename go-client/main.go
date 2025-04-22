//go:build linux

// This program demonstrates how to attach an eBPF program to a tracepoint.
// The program is attached to the syscall/sys_enter_openat tracepoint and
// prints out the integer 123 every time the syscall is entered.
package main

import (
	"errors"
	"io"
	"log"
	"net/http"
	"net/url"
	"os"
	"os/signal"
	"syscall"

	"github.com/cilium/ebpf/link"
	"github.com/cilium/ebpf/perf"
	"github.com/cilium/ebpf/rlimit"
)

//go:generate go run github.com/cilium/ebpf/cmd/bpf2go -type event -tags linux bpf tracepoint.c -- -I./headers

const uri = "http://127.0.0.1:5001"

func main() {
	// Subscribe to signals for terminating the program.
	stopper := make(chan os.Signal, 1)
	signal.Notify(stopper, os.Interrupt, syscall.SIGTERM)

	// Allow the current process to lock memory for eBPF resources.
	if err := rlimit.RemoveMemlock(); err != nil {
		log.Fatal(err)
	}

	// Load pre-compiled programs and maps into the kernel.
	objs := bpfObjects{}
	if err := loadBpfObjects(&objs, nil); err != nil {
		log.Fatalf("loading objects: %v", err)
	}
	defer objs.Close()

	// Open a perf reader from userspace into the perf event array
	// created earlier.
	rd, err := perf.NewReader(objs.Events, os.Getpagesize())
	if err != nil {
		log.Fatalf("creating event reader: %s", err)
	}
	defer rd.Close()

	// Close the reader when the process receives a signal, which will exit
	// the read loop.
	go func() {
		<-stopper
		rd.Close()
	}()

	// Open a tracepoint and attach the pre-compiled program. Each time
	// the kernel function enters, the program will increment the execution
	// counter by 1. The read loop below polls this map value once per
	// second.
	// The first two arguments are taken from the following pathname:
	// /sys/kernel/tracing/events/kmem/mm_page_alloc
	kp, err := link.Tracepoint("syscalls", "sys_enter_execve", objs.SysEnterExecve, nil)
	if err != nil {
		log.Fatalf("opening tracepoint: %s", err)
	}
	defer kp.Close()

	log.Println("Waiting for events..")

	for {
		record, err := rd.Read()
		if err != nil {
			if errors.Is(err, perf.ErrClosed) {
				log.Println("Received signal, exiting..")
				return
			}
			log.Printf("reading from reader: %s", err)
			continue
		}

		// Currently we don't get the whole processes information from ebpf
		// That is why we only print the result and don't send them to the server yet
		log.Println("Record:", record)
		log.Println("Argument:", string(record.RawSample[:]))
	}
}

func getHello() {
	var client http.Client
	resp, err := client.Get(uri + "/")
	if err != nil {
		log.Fatal(err)
	}
	defer resp.Body.Close()

	if resp.StatusCode == http.StatusOK {
		bodyBytes, err := io.ReadAll(resp.Body)
		if err != nil {
			log.Fatal(err)
		}
		bodyString := string(bodyBytes)
		log.Printf("%s", bodyString)
	}
}

func sendCommand() {
	resp, err := http.PostForm(uri+"/command",
		url.Values{"userid": {"3"}, "command": {"whoami"}})
	if err != nil {
		log.Fatal(err)
	}
	defer resp.Body.Close()

	log.Printf("%d", resp.StatusCode)
	if resp.StatusCode == http.StatusOK {
		bodyBytes, err := io.ReadAll(resp.Body)
		if err != nil {
			log.Fatal(err)
		}
		bodyString := string(bodyBytes)
		log.Printf("%s", bodyString)
	}
}
