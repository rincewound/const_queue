# Const_Queue
Const_Queue is an implementation of a queue backed by an array, that is suitable for environments where no standard library exists (only libcore). 

## Implementation Details
Const_Queue implements a ringbuffer to store its items. The implementation will always leave one free slot between the last and the first item of the buffer. This means, that 
´ConstQueue<T,4>´ will effectively only hold 3 items.

## Example

```rust
    let mut q = ConstQueue::<i32, 3>::new();
    let _ = q.push(10);
    let _ = q.push(20);
    assert!(q.pop().unwrap() == 10);
    assert!(q.pop().unwrap() == 20);
```

We also support iterators:
```rust
    let mut q = ConstQueue::<i32, 4>::new();
    let _ = q.push(10);
    let _ = q.push(20);
    let mut values = Vec::<i32>::new();
    for i in q.into_iter()
    {
        values.push(i);
    }

    assert!(values == vec![10,20]);
```