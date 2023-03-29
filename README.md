# shardedmemory
Rust node for distributed memory system, based on Attiya's 1990 paper on shared registers. Algorithm Simple is a generalization of the algorithm developed by Attiya et al. for the MWMR
environment. 

## Protocol
Write: The write operation performs two communication rounds. 

In the first round the writer sends query messages to all the servers which are alive (determined by the fail detection system). 

During the second round the writer performs the following three steps:(i) it discovers the pair with the maximum tag among
the replies received in the first round, (ii) it generates a new tag by incrementing the tag inside
the maximum discovered tag, and (iii) it propagates the new tag along with the value to be written to the active set of servers.

Read: Unlike the original Attiya protocol, the use of a fail safe allows for the read to only use one round.

It discovers the pair with the maximum tag among the replies received in the first round and returns to the client. At this point, no person reading the network would receive a value that preceedes this one, because at least one server has the most recent value and any subsequent reads will see it. 

## Fail Detection
The protocol uses a simple fail detection system rather than relying on a majority quourum. A separate thread in each node will ping all other nodes every 4 seconds for a pong() ACK. If this results in an error, then the server will remove the error address from it's peer list. 

In order to satisfy concurrency requiremnets, the peer list is only accessed once every 4 seconds. 

## Sharding
This system has three nodes for testing purposes, so I did not include the parameter f. I set it implicitly to f = 2/3. because 1 would not have redundancy and 3 would not demonstrate sharding. the db.rs file contains a HashFunction type which wraps the hashring external crate. the HashFunction is hardcoded to have three virtual clusters of nodes with two nodes each. This could be extended to be dynamic based on parameters. The HashFunction supports adding and removing nodes.


## Design
This project is built on the following open-source libraries:

- 'Tokio' - rust runtime environment good for thread execution
- 'serde' - serialization tools for structs and json interactions
- 'warp' - high level server library built on hyper to expose HTTP
- 'reqwest' - clinet library built on hyper which makes internal API calls between nodes
- 'hashring' - use of a lightweight consistent hashing implementation for sharding

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

the database is sharded, so if you want to play with that you can use the following addresses which I have tested will result in these clusters:

addr 1 => 3030,3031
addr 3 => 3030, 3032

example testing flow:
POST localhost:3030/write with body {addr: 1, value: "I am server 1"}
GET localhost:3032/registers/1 #this is an internal api call for the node's memory SHOULD BE NIL 
GET localhost:3031/registers/1 should have value
GET loclahost:3032/read/1 #public facing user api should grab the value! 

## Improvements
- Adding a timeout to fail detection system to avoid waitting if a server freezes. Currently an Error is sufficient to detect crashes, but a frozen api call which is more likely would halt the detection system breaking everything. 
- Re-use a single client instance in server to server http requests to increase speed.
- Replace Reqwest and Warp for Hyper so the server can seamlessly have a client integrated into it's handlers. Currently the use of a higher-level client library is causing irregular patters in the handler module. This would also help readability and performance. 
- peer discovery for graceful addition of nodes to the network. internal API's for editing the peer list.
- use of one persistent database rather than a series of Arc Mutex patterns for node-wide information.

## locality based scheduling

The following assumes a job processing functionality and endpoint in the node - where nodes can pass jobs to each other and return the results to the caller.

Given the known addresses of the task, we must design a scheduler intended to increase the system's efficiency. The scheudler should have low global overhead, be flexible to the system topology changing, and scalable meaning it performs as well with more nodes. 

Thus, the proposed algorithm is built upon the hashring already present in the node, since the hashring satisfies the properties of being lightweight, flexible, and scalable. 

- job sent to node with a set address space
- for each address
    - vcluster = hashring.get(address)
    - hashmap iterate through vcluster
    - best_node = argmax(hashmap)
- send job to best_node

This algorithm grows in time based on the job size and number of nodes in a virtual cluster. in a huge system, the number of nodes in a cluster can grow very fast. 

"threads are assigned to the processor which is close to the data accessed by the thread. This policy aims to reduce processor waiting time due to cache misses. Fairness among threads of the same application is not necessary,"

