use std::slice::Chunks;


#[derive(Debug)]
pub struct TwoDArray<T> {
    array : Vec::<Option<T>>,
    width: usize,
    height: usize

}

impl<T:std::fmt::Debug+Clone+std::fmt::Display> TwoDArray<T> {

    pub fn new(width: usize, height: usize) -> TwoDArray<T> {
        let mut array = Vec::<Option<T>>::new();
        let total_size = width * height;
        for _index in 0..total_size  {
            array.push(None);
        }

        TwoDArray { array: array, width: width, height: height }
    }

    pub fn get(&self,x: usize, y: usize) -> Option<T> {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.array[index].clone()
        }
        else {
            None
        }
    }

    pub fn get_string(&self,x: usize, y: usize) ->  String{
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            match &self.array[index] {
                Some(val) => format!("{}",val),
                None =>   format!("{}","N"),
            }
        }
        else {
                format!("{}","N")
        }
    }


    pub fn get_row_iter(&self) -> Chunks<'_, Option<T>> {
        self.array.chunks(self.width)
    }


    pub fn set(&mut self, x : usize, y : usize , value : T)  {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.array[index] = Some(value);
        }
    }

    pub fn unset(&mut self, x : usize, y : usize )  {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.array[index] = None;
        }
    }

    pub fn log_display(&self) {

        let header : String = (0..self.width).map(|val| format!("{:2} ",val)).collect();
        println!("{:10} {}","Col",header);
        for row in 0..self.height {
            let mut row_data = Vec::new();
            for col in 0..self.width {
                row_data.push(format!("{:2} ",self.get_string(col,row)));
            }
            let row_format : String = row_data.join("");
            println!("Row {:2} :    {}", row,row_format);
        }

    }
}


#[cfg(test)]
mod array_test {

    use crate::TwoDArray;

    #[test]
    fn flattend_array_basic() {
        let size = 8; 
        let mut data : TwoDArray<u32> = TwoDArray::new(size, size);


        data.set(1,2,12);
        data.set(2,3,23);
        data.set(3,4,34);
        data.set(6,7,67);
        data.set(8,8,88);
        for i in 0..size {
            assert_eq!(data.get(i,i), None)
        }
        assert_eq!(data.get(1,2),Some(12));
        assert_eq!(data.get(2,3),Some(23));
        assert_eq!(data.get(3,4),Some(34));
        assert_eq!(data.get(6,7),Some(67));
        assert_eq!(data.get(8,8),None);
    }

}

