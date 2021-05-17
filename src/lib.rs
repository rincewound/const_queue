
#[derive(Debug, PartialEq)]
pub enum QueueErr
{
    QueueFull,
    Empty,
    UnknownError
}

pub struct ConstQueue<Ty, const SIZE: usize>
{
    buff: [Option<Ty>; SIZE],
    start: usize,
    end: usize,
}

impl <Ty, const SIZE: usize> ConstQueue<Ty, SIZE>
{
    pub fn new() -> Self
    {
        unsafe {
            Self
            {
                buff: core::mem::zeroed(),
                start: 0,
                end: 0
            }
        }
    }

    /// Yields true, if the queue is empty
    pub fn empty(&self) -> bool
    {
        let first = self.peek();
        if let Err(e) = first
        {
            return e == QueueErr::Empty;
        }
        return false;
    }

    /// Tries to push a given item into the queue
    /// # Arguments
    /// * `item`: The item to push
    ///
    /// Will Result Err(QueueErr::Full) if the queue is full
    pub fn push(&mut self, item: Ty) -> Result<(), QueueErr>
    {
        let next_end = (self.end + 1) % SIZE;
        if next_end == self.start
        {
            return Err(QueueErr::QueueFull);
        }

        self.buff[self.end] = Some(item);
        self.end = next_end;
        Ok(())
    }

    /// Tries to forcibly push a given item into the queue
    /// # Arguments
    /// * `item`: The item to push
    ///
    /// Will panic if the queue is full
    pub fn force_push(&mut self, item: Ty)
    {
        if self.push(item).is_err()
        {
            panic!("Queue is full");
        }
    }

    /// Returns an immutable reference to the first element
    /// of the queue or Err(QueueErr::Empty), if no
    /// element is in the queue
    pub fn peek(&self) -> Result<&Ty, QueueErr>
    {
        if self.start < self.end
        {
            let v = &self.buff[self.start];
            match v
            {
                Some(ref x) => return Ok(x),
                _ => return Err(QueueErr::UnknownError) /* This should never happen */
            }
        }
        if self.start > self.end
        {
            let v = &self.buff[self.start];
            match v
            {
                Some(ref x) => return Ok(x),
                _ => return Err(QueueErr::UnknownError) /* This should never happen */
            }
        }
        return Err(QueueErr::Empty);
    }

    /// Removes the first item in the queue and returns it.
    /// Will return Err(QeueErr::Empty) if no item is in the queue
    pub fn pop(&mut self) -> Result<Ty, QueueErr>
    {
        if self.start == self.end
        {
            return Err(QueueErr::Empty)
        }

        let next_start = (self.start + 1) % SIZE;
        let result =  Ok(self.buff[self.start].take().unwrap());
        self.start = next_start;
        return result;
    }

}

impl <Ty, const SIZE: usize> Iterator for ConstQueue<Ty, SIZE> 
where Ty: Copy
{
    type Item = Ty;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.pop();
        if item.is_ok()
        {
            return Some(item.unwrap());
        }
        else
        {
            return None;
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::ConstQueue;

    #[test]
    pub fn can_push_value()
    {
        let mut q = ConstQueue::<i32, 4>::new();
        assert!(q.push(1).is_ok())
    }

    #[test]
    pub fn empty_yields_true_if_empty()
    {
        let q = ConstQueue::<i32, 3>::new();
        assert!(q.empty())
    }

    #[test]
    pub fn empty_yields_false_if_not_empty()
    {
        let mut q = ConstQueue::<i32, 3>::new();
        let _= q.push(1);
        assert!(!q.empty())
    }

    #[test]
    pub fn empty_yields_true_if_emptied()
    {
        let mut q = ConstQueue::<i32, 3>::new();
        let _= q.push(1);
        let _= q.pop();
        assert!(q.empty())
    }

    #[test]
    pub fn push_fails_if_queue_full()
    {
        /*
            The queue will always leave one slote between end and start, 
            if end is coming up from behind. 
         */
        let mut q = ConstQueue::<i32, 3>::new();
        assert!(q.push(1).is_ok());
        assert!(q.push(2).is_ok());
        assert!(q.push(3).is_err());
    }

    #[test]
    pub fn can_peek_value()
    {
        let mut q = ConstQueue::<i32, 4>::new();
        let _= q.push(1);
        let _= q.push(2);
        assert!(*q.peek().unwrap() == 1);
    }

    #[test]
    pub fn can_push_after_warparound()
    {
        let mut q = ConstQueue::<i32, 4>::new();
        let _= q.push(1);
        let _= q.push(2);
        let _= q.push(3);
        let _ = q.pop();
        assert!(q.push(4).is_ok());
    }

    #[test]
    pub fn peek_fails_if_empty()
    {
        let q = ConstQueue::<i32, 4>::new();
        assert!(q.peek().is_err());
    }

    #[test]
    pub fn can_pop_value()
    {
        let mut q = ConstQueue::<i32, 3>::new();
        let _ = q.push(10);
        let _ = q.push(20);
        assert!(q.pop().unwrap() == 10);
        assert!(q.pop().unwrap() == 20);
    }

    #[test]
    pub fn pop_fails_if_empty()
    {
        let mut q = ConstQueue::<i32, 4>::new();
        assert!(q.pop().is_err());       
    }

    #[test]
    pub fn can_iterate_over_items()
    {
        let mut q = ConstQueue::<i32, 4>::new();
        let _ = q.push(10);
        let _ = q.push(20);
        let mut values = Vec::<i32>::new();
        for i in q.into_iter()
        {
            values.push(i);
        }

        assert!(values == vec![10,20]);
    }

}