# shardedmemory
Rust node for a sharded memory system with job scheduler

## Status
Milestones 1 and 2 are almost done, system is working but needs fail detection

sharding to come next

## Improvements
- Expand coverage of proper error handling on edge cases. 
- Re-use a single client instance in server to server http requests.
- Replace Reqwest and Warp for Hyper so the server can seamlessly have a client integrated into it's handlers. Currently the use of a higher-level client library is causing irregular patters in the handler module. 
