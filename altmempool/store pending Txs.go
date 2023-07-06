package main

import (
	"container/heap"
	"fmt"
)

// Transaction represents an Ethereum transaction
type Transaction struct {
	Hash              string
	GasPrice          uint64
	Nonce             uint64
	Data              []byte
	Recipient         string
	VectorCommitment  string
	// Add other transaction fields as needed
}

// PriorityQueue implements a priority queue for transactions based on gas price
type PriorityQueue []*Transaction

// Len returns the number of transactions in the priority queue
func (pq PriorityQueue) Len() int { return len(pq) }

// Less compares the gas price of two transactions
func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].GasPrice > pq[j].GasPrice // Higher gas price has higher priority
}

// Swap swaps the position of two transactions in the priority queue
func (pq PriorityQueue) Swap(i, j int) { pq[i], pq[j] = pq[j], pq[i] }

// Push adds a transaction to the priority queue
func (pq *PriorityQueue) Push(x interface{}) {
	item := x.(*Transaction)
	*pq = append(*pq, item)
}

// Pop removes and returns the transaction with the highest priority (highest gas price)
func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	*pq = old[0 : n-1]
	return item
}

// GetTransactionByHash retrieves a transaction from the priority queue based on its hash
func (pq PriorityQueue) GetTransactionByHash(hash string) *Transaction {
	for _, transaction := range pq {
		if transaction.Hash == hash {
			return transaction
		}
	}
	return nil
}

// RemoveTransactionByHash removes a transaction from the priority queue based on its hash
func (pq *PriorityQueue) RemoveTransactionByHash(hash string) {
	index := -1
	for i, transaction := range *pq {
		if transaction.Hash == hash {
			index = i
			break
		}
	}
	if index >= 0 {
		*pq = append((*pq)[:index], (*pq)[index+1:]...)
	}
}

func main() {
	// Create an empty priority queue for pending transactions
	pq := make(PriorityQueue, 0)

	// Push some sample transactions into the priority queue
	pq.Push(&Transaction{
		Hash:              "tx1",
		GasPrice:          100,
		Nonce:             0,
		Data:              []byte("transaction data"),
		Recipient:         "0x1234567890abcdef",
		VectorCommitment:  "0xabcdef1234567890",
	})
	pq.Push(&Transaction{
		Hash:              "tx2",
		GasPrice:          50,
		Nonce:             1,
		Data:              []byte("transaction data"),
		Recipient:         "0xabcdef1234567890",
		VectorCommitment:  "0x67890abcdef12345",
	})
	pq.Push(&Transaction{
		Hash:              "tx3",
		GasPrice:          200,
		Nonce:             2,
		Data:              []byte("transaction data"),
		Recipient:         "0x67890abcdef12345",
		VectorCommitment:  "0x1234567890abcdef",
	})

	// Pop transactions from the priority queue in order of gas price
	for pq.Len() > 0 {
		transaction := heap.Pop(&pq).(*Transaction)
		fmt.Printf("Transaction Hash: %s```
		fmt.Printf("Gas Price: %d\n", transaction.GasPrice)
		fmt.Printf("Nonce: %d\n", transaction.Nonce)
		fmt.Printf("Recipient: %s\n", transaction.Recipient)
		fmt.Printf("Vector Commitment: %s\n", transaction.VectorCommitment)
		fmt.Println("------------------")
	}

	// Get transaction by hash
	hash := "tx2"
	foundTransaction := pq.GetTransactionByHash(hash)
	if foundTransaction != nil {
		fmt.Printf("Transaction found: %s\n", foundTransaction.Hash)
	} else {
		fmt.Printf("Transaction not found: %s\n", hash)
	}

	// Remove transaction by hash
	removeHash := "tx3"
	pq.RemoveTransactionByHash(removeHash)
	fmt.Printf("Transaction removed: %s\n", removeHash)

	// Pop remaining transactions after removal
	fmt.Println("Remaining transactions:")
	for pq.Len() > 0 {
		transaction := heap.Pop(&pq).(*Transaction)
		fmt.Printf("Transaction Hash: %s\n", transaction.Hash)
		fmt.Printf("Gas Price: %d\n", transaction.GasPrice)
		fmt.Printf("Nonce: %d\n", transaction.Nonce)
		fmt.Printf("Recipient: %s\n", transaction.Recipient)
		fmt.Printf("Vector Commitment: %s\n", transaction.VectorCommitment)
		fmt.Println("------------------")
	}
}
