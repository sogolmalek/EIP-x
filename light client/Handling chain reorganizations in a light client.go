package main

import (
	"context"
	"fmt"
	"log"
	"math/big"
	"time"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/ethereum/go-ethereum/rpc"
)

type Header struct {
	ParentHash string
	StateRoot  string
	Number     *big.Int
}

type LightClient struct {
	Headers []*Header
}

func NewLightClient() *LightClient {
	return &LightClient{
		Headers: []*Header{},
	}
}

func (lc *LightClient) AddHeader(header *Header) {
	lc.Headers = append(lc.Headers, header)
}

func (lc *LightClient) HandleChainReorg(client *ethclient.Client) error {
	latestHeader, err := client.HeaderByNumber(context.Background(), nil)
	if err != nil {
		return err
	}

	reorgIndex := -1
	for i, header := range lc.Headers {
		if header.Number.Cmp(latestHeader.Number) > 0 {
			// Found a header with a higher block number than the latest
			reorgIndex = i
			break
		} else if header.Number.Cmp(latestHeader.Number) == 0 && header.ParentHash != latestHeader.ParentHash {
			// Found a competing header with the same block number but different parent hash
			reorgIndex = i
			break
		}
	}

	if reorgIndex >= 0 {
		// Remove headers after the reorganization point
		lc.Headers = lc.Headers[:reorgIndex]

		// Request headers in the reorganized chain
		start := lc.Headers[len(lc.Headers)-1].Number
		end := latestHeader.Number

		headers, err := fetchHeadersInRange(client, start, end)
		if err != nil {
			return err
		}

		// Validate and store the reorganized headers
		if err := validateAndStoreHeaders(headers); err != nil {
			return err
		}

		fmt.Printf("Chain reorganization detected! Switched to chain with new headers starting from block number %s\n", start.String())
	} else {
		fmt.Println("No chain reorganization detected.")
	}

	return nil
}

func fetchHeadersInRange(client *ethclient.Client, start, end *big.Int) ([]*Header, error) {
	headers := []*Header{}
	for i := start.Uint64(); i <= end.Uint64(); i++ {
		blockNumber := big.NewInt(int64(i))
		header, err := client.HeaderByNumber(context.Background(), blockNumber)
		if err != nil {
			return nil, err
		}
		headers = append(headers, &Header{
			ParentHash: header.ParentHash.String(),
			StateRoot:  header.StateRoot.String(),
			Number:     header.Number,
		})
	}
	return headers, nil
}

func validateAndStoreHeaders(headers []*Header) error {
	// Validate and store the headers in the appropriate order
	for _, header := range headers {
		// Perform header validation, e.g., parent-child relationship, POW, etc.
		// Store the header in the appropriate data structure or database
		fmt.Printf("Storing header %s\n", header.Number.String())
	}
	return nil
}

func main() {
	// Connect to Ethereum node
	client, err := ethclient.Dial("http://localhost:8545")
	if err != nil {
		log.Fatal(err)
	}

	// Create a new light client
	lightClient := NewLightClient()

	// Synchronize headers in batches
	batchSize := 100
	for start := new(big.Int).SetUint64(0); ; start.Add(start, big.NewInt(batchSize)) {
		end := new(big.Int).Add(start, big.NewInt(batchSize))

		// Request headers in the current batch
		headers, err := fetchHeadersInRange(client, start, end)
		if err != nil {
			log.Fatal(err)
		}

		// Validate and store the headers
		if err := validateAndStoreHeaders(headers); err != nil {
			log.Fatal(err)
		}

		// Add headers to the light client
		for _, header := range headers {
			lightClient.AddHeader(header)
		}

		// Handle chain reorganization
		if err := lightClient.HandleChainReorg(client); err != nil {
			log.Fatal(err)
		}

		// Delay between batches to avoid overwhelming the Ethereum node
		time.Sleep(time.Second * 5)
	}

	fmt.Println("Header synchronization and chain reorganization handling completed.")
}
