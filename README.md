# shardedmemory
Rust node for distributed memory system, based on Attiya's 1990 paper on shared registers. Algorithm Simple is a generalization of the algorithm developed by Attiya et al. for the MWMR
environment. 

## Protocol
Each node receives read and write requests, and updates its local copy of the
replica if the tag enclosed in the received message is greater than its local tag before replying with an
acknowledgment and its local copy to the requester.
Write: The write operation performs two communication rounds. In the first round the writer
sends query messages to all the servers which are alive (determined by the fail detection system). During the second
round the writer performs the following three steps: (i) it discovers the pair with the maximum tag among
the replies received in the first round, (ii) it generates a new tag by incrementing the tag inside
the maximum discovered tag, and (iii) it propagates the new tag along with the value to be written to the active set of servers.
Read: Unlike the original Attiya protocol, the use of a fail safe allows for the read to only use one round.
The round is identical as the first round of a write operation. It discovers the pair with the maximum tag among the
replies received in the first round and returns to the client. At this point, no person reading the network would receive a value that preceedes this one, because at least one server has the most recent value and any subsequent reads will see it. 

## Fail Detection
The protocol uses a simple fail detection system rather than relying on a majority quourum. A separate thread in each node will ping all other nodes every 2 seconds for a pong() ACK. If this results in an error, then the server will remove the errored address from it's peer list. 

## Design
This project is built on the following open-source libraries:

- 'Tokio' - rust runtime environment good for thread execution
- 'serde' - serialization tools for structs and json interactions
- 'warp' - high level server library built on hyper to expose HTTP
- 'reqwest- - clinet library built on hyper which makes internal API calls between nodes

These are the files in src:
- 'db.rs': wrappers for the data storage of peers and table entries
- 'models.rs': struct definitions for table entries
- 'handlers.rs': the business logic functions for both internal api calls and public api 
- 'routes.rs': file which uses warp filters to connect endpoints to actual business logic
- 'workers.rs': contains the fail detection system which is instantiated in a child thread from the server

each node stores a HashMap of address (int) to Val (struct containing a tag and a string). These make up an Entry in the node's local database. Thus an Entry is the umbrella struct used to communicate information between nodes. it contains everything needed to maintain this distriburted system. read_register() and update_register() functions are internal functions which assist the protocol in state replication. read() and write() are the main external facing fucntions which satisfy the user API. 

## Usage

open three terminals in the sharedmemory directory. the repo currently hardcodes peers to be localhosted at ports 3030,3031,3032

`cargo run 3030`
`cargo run 3031`
`cargo run 3032`

run the above commands (in quick succession so that the fail detection system does not accidentally trigger). The API is specified as below:

- GET localhost:{port}/read/{addr} 
- POST localhost:{port}/write with body {addr: int, value: string}

and the system will do the rest for you! If you want a postman collection here is mine: 
https://api.postman.com/collections/26594267-00b965b2-ea19-45ec-9bed-c1fc7a128e84?

## Improvements
- Adding a timeout to fail detection system to avoid waitting if a server freezes. Currently an Error is sufficient to detect crashes, but a frozen api call which is more likely would halt the detection system breaking everything. 
- Re-use a single client instance in server to server http requests to increase speed.
- Replace Reqwest and Warp for Hyper so the server can seamlessly have a client integrated into it's handlers. Currently the use of a higher-level client library is causing irregular patters in the handler module. This would also help readability and performance. 
- graceful dynamic addition and subtraction of nodes in the network.
