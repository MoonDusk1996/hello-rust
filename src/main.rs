mod services;
use services::soma_dois_numeros::soma_dois_numeros;
use services::hello_world::hello_world;

fn main() {
    hello_world();
    soma_dois_numeros(32656465, 45456456);
}