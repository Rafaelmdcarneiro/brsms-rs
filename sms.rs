fn main() {
    let mut cpu = Z80::new();   
    loop {
        cpu.execute_next();
    }
}
