In this modified code, Iv'e created a LightClient struct that holds the Header of the light client. The NewLightClient function is used to create a new instance of the light client with the specified header.

The VerifyStateInclusionProof method of the LightClient struct takes a leaf and a proof as inputs and verifies the inclusion proof against the Merkle tree built from the proof and the root stored in the header. It returns true if the proof is valid, and false otherwise.

In the main function, we create a Header and a LightClient instance. We then use the VerifyStateInclusionProof method to verify the state inclusion proof for a given leaf and proof.
