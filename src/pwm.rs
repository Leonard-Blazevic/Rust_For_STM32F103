// Use the `pub` modifier to override default visibility.
pub fn intensity(received_value: u8) -> u16 {
	let mut result : u16 = 0;
    match received_value {
	    b'0' => result = 0,
	    b'1' => result = 300,
	    b'2' => result = 700,
	    b'3' => result = 1200,
	    b'4' => result = 2000,
	   	b'5' => result = 3000,
	    b'6' => result = 4000,
	    b'7' => result = 5500,
	    b'8' => result = 7000,
	    b'9' => result = 8000,
	    _ => result = 0,
	};

	result
}
