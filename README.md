# gdocs

Simple Google docs replica <br>

# Execution
Have to open up 2 terminals, one to run the server and one to run the client <br>

In one terminal, execute:
```cd server && cargo run```
<br>
In another execute
```cd gdocs && cargo test {test_fn}```
<br>
Check the bottom of main.rs for the test names

<br>

# Updates needed
1. Server hangs on reads for some reason. I think it has to do with the amount of bytes our stream
   is taking in, vs how much we are expecting to recieve? <br>
2. Move to wasm instead of cli
