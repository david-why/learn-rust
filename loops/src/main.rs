fn main() {
    let x = [10, 20, 30, 40, 50];

    let y = 'label: loop {
        for item in x {
            break 'label item;
        };
    };
}
