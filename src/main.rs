//Code to remove duplicates

fn print (a : &mut [i32], len:usize) {

    let mut i = 0;
    let mut j = 0;

    loop {
        if a[i] != 0 {
            a[j] = a[i];
            j += 1;
        }

        i += 1;
        if i == len {
            break;
        }
    }
    i = 0;
    loop {

        println! ("value = {}",a[i]);
        i += 1;
        if i == j {
            break;
        }
    }
}

fn main() {
    let mut a = [1,1,1,2,2,2,3,3,3,4,4,4,4];
    let count = a.len();
    let len = a.len() - 1;
    let mut i = 1;
    loop {

        if a[i-1] == a[i] {
            a[i-1] = 0;
        }
        i += 1;

        if i == count - 1 {
           break;//  'counting_up';
        }
    }

    print (&mut a, len);
}
