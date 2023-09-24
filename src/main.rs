// 11.2.2
//🌟🌟 A Vec can be extended with extend method
fn main() {
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop(); // pop usuwa ostatni element wektora (zwraca też go)
    v1.push(3);
    
    let mut v2 = Vec::new(); //Domyślny typ zmiennych w vec to i32
    v2.extend(&v1); // Po prostu wpidać exted i dać argument (pożyczamy wektor)

    assert_eq!(v1, v2);

    println!("Success!");
}
