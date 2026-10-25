mod zeo;
mod unrestricted;
mod network;
mod javascript;
mod sqlite;
mod bridge;
mod test;
fn main() {
    //Module Init
    network::init(); //Networking Components (I hate this bruh )
    unrestricted::init(); //Core Browser Components
    zeo::init(); //Cybersecurity & Tooling Component (X-Mode)
    javascript::init(); //Javascript Components
    sqlite::init(); //Database
    bridge::init();//Potentially sharable Data
    test::init();//Tests for SRC Modules
}