// List ::= Null | (node v list)
 
use std::rc::Rc;

enum List {
    Empty, 
    Node(i64, Rc<List>),   // need to be sure we know where the list is allocated
} 

impl List{
    fn new() -> Self{   // self as a parameter differentiates between static or instance methods
                        // Self as return type just saves some typing
        List::Empty
    }

    fn len() -> i32{
        0
    }

    fn push_front(&self , n: i64) -> Self{

        // if the list is empty, make a new node with a null pointer
        // using pattern matching
        match *self { // de reference self to pattern match??
            List::Empty => List::Node(n, Rc::new(List::Empty)),   // make a new node that points to
                                                                  // an empty node and has value n
                                                                  
            Node(val, tail) =>  List::Node(n, Rc::new(List::Node(val, tail)))

        }
    }
}


fn main(){
    let mut lst = List::new();

    lst = lst.push_front(3);

    println!("{:?}", lst);


}
