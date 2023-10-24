package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"math/rand"
	"os"
	"time"

	"github.com/filecoin-project/go-address"
	c2proxy_go "github.com/hunjixin/gpuproxy/gpuproxy-go"
)

type Commit2In struct {
	SectorNum  int64
	Phase1Out  []byte
	SectorSize uint64
	Miner      address.Address
}

func main() {
	ctx := context.TODO()
	client, closer, err := c2proxy_go.NewC2ProxyClient(ctx, "http://127.0.0.1:20000")
	if err != nil {
		log.Fatal(err)
		return
	}
	defer closer()

	var commit2In Commit2In
	eightMiB, err := os.ReadFile("example/32G.iB.json")
	if err != nil {
		log.Fatal(err)
		return
	}
	err = json.Unmarshal(eightMiB, &commit2In)
	if err != nil {
		log.Fatal(err)
		return
	}

	var proverId [32]byte
	copy(proverId[:], commit2In.Miner.Payload())
	seedrand := rand.New(rand.NewSource(time.Now().Unix()))
	miner, _ := address.NewIDAddress(uint64(seedrand.Uint32()))
	taskId, err := client.SubmitC2Task(commit2In.Phase1Out, miner.String(), "", proverId, commit2In.SectorNum)
	if err != nil {
		log.Fatal(err)
		return
	}

	fmt.Println(taskId)
	return

	for {
		task, err := client.GetTask(ctx, taskId)
		if err != nil {
			log.Fatal(err)
			return
		}
		if task.State == c2proxy_go.Completed {
			log.Println("task ", task.Id, " has been complete by ", task.WorkerId)
			break
		}
		time.Sleep(time.Second * 5)
	}
}
