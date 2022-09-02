use two_d_array::{TwoDArray};

fn main() { 
    env_logger::init();
    let vertexes = 7;
    let iterations = vertexes-1;


    println!("\n--------- Flattened Array ------- \n");
    let mut data : TwoDArray<u32> = TwoDArray::new(vertexes, iterations,u32::MAX);

    data.set(3,0,44);
   
    let mut count = 0;
    let header : String = (0..vertexes).map(|val| format!("{:2} ",val)).collect();
    println!("{:17} {}","Vertex",header);
    for row in 0..iterations {
        let mut row_data = Vec::new();
        for col in 0..vertexes {
            row_data.push(format!("{:2} ",data.get(col,row).unwrap()));
        }
        let row_format : String = row_data.join("");
        println!("Iteration {:2} :    {}", row,row_format);
    }

    println!("Iter 0,Vertex 3 -> {}", data.get(3,0).unwrap());
    println!("Iter 1,Vertex 3 -> {}", data.get(3,1).unwrap());

    println!("Iter 4,Vertex 1 -> {}", data.get(1,4).unwrap());
    println!("Iter 6,Vertex 5 -> {}", data.get(6,5).unwrap());

    println!("\nAs Interator \n");
        
    for row in data.get_row_iter() {
        let min = row.iter().min().unwrap();
        let row_format : String = row.iter().map(|val| format!("{:>2} ",val) ).collect();
        println!("Iteration {:2} :    {}  Min: {}", count,row_format, min);
        count += 1
    }

}

