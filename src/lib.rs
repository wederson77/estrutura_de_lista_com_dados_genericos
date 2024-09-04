struct Lista<T>{
    elementos: Vec<T>,
}

impl<T> Lista<T>{
    fn new() -> Self{
        Lista{elementos: Vec::new()}
    }

    fn add(&mut self, elemento: T){
        self.elementos.push(elemento);
    }

    fn remove(&mut self, indice: usize) -> Option<T>{
        if indice < self.elementos.len(){
            Some(self.elementos.remove(indice))
        }else{
            None
        }
    }

    fn search(&mut self, elemento: &T) -> Option<usize> where T: PartialOrd,{
        self.elementos.iter().position(|e| e == elemento)        
    }

    fn sort(&mut self) where T: Ord,{
        self.elementos.sort();
    }
        
    
}

#[test]
fn test_add_and_search(){
    let mut list = Lista::new();
    list.add(5);
    list.add(10);
    list.add(15);
    list.add(1);
    assert_eq!(list.search(&15), Some(2));
    
}

#[test]
fn test_remove(){
    let mut list = Lista::new();
    list.add(5);
    list.add(10);
    list.add(15);
    list.add(1);
    list.remove(2);
    assert_eq!(list.elementos, vec![5, 10, 1]);
    
}