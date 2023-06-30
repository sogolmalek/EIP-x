package main

import (
	"crypto/sha256"
	"fmt"
	"sync"
)

type MerkleTree struct {
	Leaves []string
	Tree   []string
}

type Header struct {
	ParentHash            string
	StateRoot             string
	StateVectorCommitment []string
}

type LightClient struct {
	Header  *Header
	Headers []*Header
	Mutex   sync.RWMutex
}

func NewMerkleTree(leaves []string) *MerkleTree {
	return &MerkleTree{
		Leaves: leaves,
		Tree:   buildTree(leaves),
	}
}

func buildTree(leaves []string) []string {
	if len(leaves) == 1 {
		return leaves
	} else {
		nextLevel := []string{}
		for i := 0; i < len(leaves); i += 2 {
			leftLeaf := leaves[i]
			rightLeaf := leftLeaf
			if i+1 < len(leaves) {
				rightLeaf = leaves[i+1]
			}
			parentHash := hashNodes(leftLeaf, rightLeaf)
			nextLevel = append(nextLevel, parentHash)
		}
		return buildTree(nextLevel)
	}
}

func hashNodes(left, right string) string {
	hashInput := left + right
	hash := sha256.Sum256([]byte(hashInput))
	return fmt.Sprintf("%x", hash)
}

func NewLightClient() *LightClient {
	return &LightClient{
		Header:  nil,
		Headers: []*Header{},
		Mutex:   sync.RWMutex{},
	}
}

func (client *LightClient) AddHeader(header *Header) {
	client.Mutex.Lock()
	defer client.Mutex.Unlock()

	client.Headers = append(client.Headers, header)
	client.Header = header
}

func (client *LightClient) ValidateHeaderChain() bool {
	client.Mutex.RLock()
	defer client.Mutex.RUnlock()

	if len(client.Headers) == 0 {
		return true
	}

	for i := 1; i < len(client.Headers); i++ {
		prevHeader := client.Headers[i-1]
		currentHeader := client.Headers[i]

		// Validate parent hash
		if hashHeader(prevHeader) != currentHeader.ParentHash {
			return false
		}

		// Validate state vector commitment
		if !client.VerifyStateVectorCommitment(currentHeader) {
			return false
		}

		// Additional validation checks for state root, etc.
	}

	return true
}

func hashHeader(header *Header) string {
	hashInput := header.ParentHash + header.StateRoot + concatStrings(header.StateVectorCommitment)
	hash := sha256.Sum256([]byte(hashInput))
	return fmt.Sprintf("%x", hash)
}

func (client *LightClient) VerifyStateVectorCommitment(header *Header) bool {
	stateVectorCommitment := header.StateVectorCommitment
	stateRoot := header.StateRoot
	merkleTree := NewMerkleTree(stateVectorCommitment)
	root := merkleTree.GetRoot()
	return root == stateRoot
}

func concatStrings(strs []string) string {
	result := ""
	for _, str := range strs {
		result += str
	}
	return result
}

func (tree *MerkleTree) GetRoot() string {
	if len(tree.Tree) > 0 {
		return tree.Tree[0]
	}
	return ""
}

func main() {
	client := NewLightClient()

	// Simulated header synchronization
	header1 := &Header{
		ParentHash:            "parent_hash_1",
		StateRoot:             "state_root_1",
		StateVectorCommitment: []string{"leaf_1", "leaf_2", "leaf_3"},
	}

	header2 := &Header{
		ParentHash:            "parent_hash_2",
		StateRoot:             "state_root_2",
		StateVectorCommitment: []string{"leaf_4", "leaf_5", "leaf_6"},
	}

	client.AddHeader(header1)
	client.AddHeader(header2)

	// Validate the header chain
	isValid := client.ValidateHeaderChain()
	if isValid {
		fmt.Println("Header chain is valid!")
	} else {
		fmt.Println("Header chain is not valid.")
	}
}
