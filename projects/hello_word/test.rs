use std::io::{self, Write};

fn ft_toupper(c: char) -> char
{
	if c >= 'a' && c <= 'z'
	{
		((c as u8) - 32) as char
	}
	else
	{
		c
	}
}


fn main() -> io::Result<()>
{
	let a: char;

	a = 'a';
	writeln!(io::stdout(), "{}", a.to_uppercase())?;
	writeln!(io::stdout(), "{}", ft_toupper(a))?;
	Ok(())
}
