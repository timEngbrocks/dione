use std::fmt::Display;

pub type U1 = u8;
pub type U2 = u16;
pub type U4 = u32;

pub struct Parser {
    data: Vec<u8>,
    cursor: usize,
}

impl Display for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("start: {:X}, length: {:X}", self.cursor, self.data.len()))
    }
}

impl Parser {
    pub fn new(data: Vec<u8>) -> Parser {
        Parser { data, cursor: 0 }
    }

    pub fn peek_u1(&self) -> U1 {
        match nom::number::streaming::be_u8::<_, (_, nom::error::ErrorKind)>(self.current_data()) {
            Ok((_, value)) => value,
            Err(error) => panic!("{error}"),
        }
    }

    pub fn peek_u2(&self) -> U2 {
        match nom::number::streaming::be_u16::<_, (_, nom::error::ErrorKind)>(self.current_data()) {
            Ok((_, value)) => value,
            Err(error) => panic!("{error}"),
        }
    }

    pub fn peek_u4(&self) -> U4 {
        match nom::number::streaming::be_u32::<_, (_, nom::error::ErrorKind)>(self.current_data()) {
            Ok((_, value)) => value,
            Err(error) => panic!("{error}"),
        }
    }

    pub fn consume_u1(&mut self) -> U1 {
        if self.length() < 1 {
            todo!("error handling");
        }
        let value = self.peek_u1();
        self.cursor += 1;
        value
    }

    pub fn consume_u2(&mut self) -> U2 {
        if self.length() < 2 {
            todo!("error handling");
        }
        let value = self.peek_u2();
        self.cursor += 2;
        value
    }

    pub fn consume_u4(&mut self) -> U4 {
        if self.length() < 4 {
            todo!("error handling");
        }
        let value = self.peek_u4();
        self.cursor += 4;
        value
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }

    pub fn remaining(&self) -> usize {
        self.current_data().len()
    }

    pub fn offset(&self) -> usize {
        self.cursor
    }

    pub fn align(&mut self, alignment: usize) -> usize {
        let padding = alignment - (self.cursor % alignment);
        if padding == alignment {
            return 0;
        }
        for _ in 0..padding {
            self.consume_u1();
        }
        padding
    }

    fn current_data(&self) -> &[u8] {
        &self.data[self.cursor..self.length()]
    }
}