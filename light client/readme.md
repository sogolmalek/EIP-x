In this code, the LightClient struct now includes an additional field Headers, which is a slice of all the headers received by the light client. The Mutex is used to synchronize access to the Headers slice.

The AddHeader method allows adding a new header to the Headers slice, and it updates the Header field of the light client to the most recent header.

The ValidateHeaderChain method has been updated to perform header chain validation. It checks the parent hash and state vector commitment of each header in the Headers slice. Additional validation checks can be added as required.

The hashHeader function computes the hash of a header by concatenating its fields.

The main function demonstrates the usage of the light client by adding headers and validating the header chain.
