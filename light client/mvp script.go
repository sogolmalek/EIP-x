package main

import (
	"crypto/sha256"
	"fmt"
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
	Header *Header
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

func NewLightClient(header *Header) *LightClient {
	return &LightClient{
		Header: header,
	}
}

func (client *LightClient) VerifyStateInclusionProof(leaf string, proof []string) bool {
	tree := NewMerkleTree(proof)
	root := tree.GetRoot()
	return tree.VerifyInclusionProof(leaf, proof, root)
}

func (tree *MerkleTree) GetRoot() string {
	if len(tree.Tree) > 0 {
		return tree.Tree[0]
	}
	return ""
}

func (tree *MerkleTree) VerifyInclusionProof(leaf string, proof []string, root string) bool {
	computedRoot := leaf
	for _, proofElement := range proof {
		computedRoot = hashNodes(computedRoot, proofElement)
	}
	return computedRoot == root
}

func main() {
	header := &Header{
		ParentHash:            "parent_hash_1",
		StateRoot:             "state_root_1",
		StateVectorCommitment: []string{"leaf_1", "leaf_2", "leaf_3"},
	}

	client := NewLightClient(header)

	// Verify state inclusion proof
	leaf := "leaf_1"
	proof := []string{"leaf_1", "leaf_2", "leaf_3"}
	isValid := client.VerifyStateInclusionProof(leaf, proof)
	if isValid {
		fmt.Println("State inclusion proof is valid!")
	} else {
		fmt.Println("State inclusion proof is not valid.")
	}
}
