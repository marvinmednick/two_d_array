use std::slice::Chunks;
use log::{error};


#[derive(Debug)]
pub struct TwoDArray<T> {
    array : Vec::<T>,
    width: usize,
    height: usize

}

impl<T:std::fmt::Debug+Clone+std::fmt::Display> TwoDArray<T> {

    pub fn new(width: usize, height: usize, initial_value: T) -> TwoDArray<T> {
        let mut array = Vec::<T>::new();
        let total_size = width * height;
        for _index in 0..total_size  {
            array.push(initial_value.clone());
        }

        TwoDArray { array: array, width: width, height: height }
    }

    pub fn get(&self,x: usize, y: usize) -> Result<T,String> {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            Ok(self.array[index].clone())
        }
        else {
            error!("Invalid Index x {} (max {}) y {} (max {})",x,y,self.width,self.height);
            Err("Invalid Index".to_string())
        }
    }

    pub fn get_row_iter(&self) -> Chunks<'_, T> {
        self.array.chunks(self.width)
    }


    pub fn set(&mut self, x : usize, y : usize , value : T)  {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.array[index] = value;
        }
    }

}


#[cfg(test)]
mod array_test {

    use crate::TwoDArray;

    #[test]
    fn flattend_array_basic() {
        let size = 8; 
        let mut data : TwoDArray<u32> = TwoDArray::new(size, size,u32::MAX);


        data.set(1,2,12);
        data.set(2,3,23);
        data.set(3,4,34);
        data.set(6,7,67);
        data.set(8,8,88);
        for i in 0..size {
            assert_eq!(data.get(i,i), Ok(u32::MAX));
        }
        assert_eq!(data.get(1,2),Ok(12));
        assert_eq!(data.get(2,3),Ok(23));
        assert_eq!(data.get(3,4),Ok(34));
        assert_eq!(data.get(6,7),Ok(67));
        assert_eq!(data.get(8,8),Err("Invalid Index".to_string()));
    }

}

