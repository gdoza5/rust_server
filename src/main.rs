fn main() {
    let servder = Server::new("127.0.0.1:8080") ;
    server.run();
}
struct Server {
    addr: String,
}