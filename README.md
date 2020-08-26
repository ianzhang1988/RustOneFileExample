[TOC]

# RustOneFileExample

example in one file

file naming patten: category_name

test.rs just for running some temporary code

## some thoughts while coding these examples
### Box Cell
thoughts about box and cell, how they are different, and use of mem::swap

**files:**  
tcp_server_frame*  
algo_binary_tree.rs

### iterator
how implement iterator for new type

**file**:  
algo_binary_tree2.rs
<font color="red">**also**</font>, when using container like vec, don't forget about xxx_mut function,  
when try to change value (or struct) inside vec

### error handle

as top application, use `anyhow::Result` to simplify error handle
as library, use `snafu::Snafu`, to create unified error

**file**:
amqp_task_queue_worker.rs