// Simple test to verify the code compiles
fn main() {
    println!("Testing phoenix-core structure...");
    
    // Test that we can create a frame header
    use phoenix_core::frame::build_frame_header;
    use bytes::Bytes;
    
    let header = build_frame_header(0, 0, 0, 0);
    assert_eq!(header.len(), 9);
    println!("Frame header construction works");
    
    // Test HPACK encoding
    use phoenix_core::frame::minimal_hpack_get_request;
    let hpack = minimal_hpack_get_request("example.com", "/");
    assert!(!hpack.is_empty());
    println!("HPACK encoding works");
    
    println!("All basic tests passed!");
}