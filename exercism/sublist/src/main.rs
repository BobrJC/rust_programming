
#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }
    else if _first_list.len() == 0 {
        return Comparison::Sublist;
    }
    else if _second_list.len() == 0 {
        return Comparison::Superlist
    }
    
    if _second_list.len() < _first_list.len() {
        for (j, el_fir) in _first_list.iter().enumerate() {
            if *el_fir == _second_list[0] {
                for k in j..j+_second_list.len() {
                    if k == _first_list.len() ||_first_list[k] != _second_list[k-j] {
                        break;
                    }
                    else {
                        if k == j+_second_list.len()-1 {
                            return Comparison::Superlist;
                        }
                    }
                }
            }
        }
    }
    else {
        for (j, el_sec) in _second_list.iter().enumerate() {
            if *el_sec == _first_list[0] {
                for k in j..j+_first_list.len() {
                    if k == _second_list.len() || _second_list[k] != _first_list[k-j] {
                        break;
                    }
                    else {
                        if k == j+_first_list.len()-1 {
                            return Comparison::Sublist;
                        }
                    }
                }
            }
        }
    }
    return Comparison::Unequal;
}




pub fn main() {
    let a = sublist(&[4, 3, 2], &[5, 4, 3, 2, 1]);
    dbg!(a);
 
}
