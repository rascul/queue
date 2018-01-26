var searchIndex = {};
searchIndex["queue"] = {"doc":"A simple and easy wrapper around `Vec` to implement a FIFO queue. This is no fancy, advanced data type but something simple you can use easily until or unless you need something different.","items":[[3,"Queue","queue","A first in, first out queue built around `Vec`.",null,null],[11,"clone","","",0,{"inputs":[{"name":"self"}],"output":{"name":"queue"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"default","","",0,{"inputs":[],"output":{"name":"queue"}}],[11,"from","","Constructs a new `Queue<T>` from a `Vec<T>`.",0,{"inputs":[{"name":"vec"}],"output":{"name":"queue"}}],[11,"new","","Constructs a new `Queue<T>`.",0,{"inputs":[],"output":{"name":"queue"}}],[11,"with_capacity","","Constructs a new `Queue<T>` with a specified capacity.",0,{"inputs":[{"name":"usize"}],"output":{"name":"queue"}}],[11,"queue","","Add an item to the end of the `Queue`. Returns `Ok(usize)` with the new length of the `Queue`, or `Err(())` if there is no more room.",0,{"inputs":[{"name":"self"},{"name":"t"}],"output":{"generics":["usize"],"name":"result"}}],[11,"force_queue","","Forcefully ad an item to the end of the `Queue`. If the `Queue` is at capacity, the first item will be removed to make room. Returns `usize` with the new length of the `Queue`.",0,{"inputs":[{"name":"self"},{"name":"t"}],"output":{"name":"usize"}}],[11,"dequeue","","Remove the next item from the `Queue`. Returns `Option<T>` so it will return either `Some(T)` or `None` depending on if there's anything in the `Queue` to get.",0,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"vec","","Return a `&Vec<T>` for the `Queue<T>`.",0,{"inputs":[{"name":"self"}],"output":{"name":"vec"}}],[11,"peek","","Peek at the next item in the `Queue`, if there's something there.",0,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"len","","The number of items currently in the `Queue`.",0,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"is_empty","","Check if the `Queue` is empty.",0,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"capacity","","Query the capacity for a `Queue`. If there is no capacity set (the `Queue` can grow as needed) then `None` will be returned, otherwise it will be `Some(usize)`.",0,{"inputs":[{"name":"self"}],"output":{"generics":["usize"],"name":"option"}}],[11,"set_capacity","","Modify the capacity of a `Queue`. If set to `None`, the `Queue` will grow automatically, as needed. Otherwise, it will be limited to the specified number of items. If there are more items in the `Queue` than the requested capacity, `Err(())` will be returned, otherwise the operation will succeed and `Ok(())` will be returned. If the capacity is shrunk, the underlying `Vec` will be shrunk also, which would free up whatever extra memory was allocated for the `Queue`.",0,{"inputs":[{"name":"self"},{"name":"c"}],"output":{"name":"result"}}]],"paths":[[3,"Queue"]]};
initSearch(searchIndex);
