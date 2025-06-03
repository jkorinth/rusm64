use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Default, Eq, PartialEq)]
pub struct Bin(pub Vec<u8>);

impl Bin {
    pub fn prg(&self) -> Vec<u8> {
        let mut start: usize = 0;
        for chunk in ChunkIterator::from(self.0.as_slice()) {
            println!("chunk = {chunk:?}");
            match chunk {
                Chunk::Repeats(0xea, n, _, _) => {
                    start += n;
                }
                _ => break,
            }
        }
        let mut end: usize = start;
        for chunk in ChunkIterator::from(&self.0[start..]) {
            match chunk {
                Chunk::Chunk(_, _, to) => {
                    end = start + to;
                }
                Chunk::Repeats(v, _, _, to) if v != 0xea => {
                    end = start + to;
                }
                _ => {}
            }
        }
        let data = &self.0[start..end];
        println!("TOTAL SIZE: {}B", data.len());
        let mut prg = Vec::<u8>::new();
        prg.resize(data.len() + 2, 0xea);
        prg[0] = (start & 0xff) as u8;
        prg[1] = ((start >> 8) & 0xff) as u8;
        prg[2..].copy_from_slice(data);
        println!("PRG SIZE: {}B", prg.len());
        prg
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Chunk<'a> {
    Repeats(u8, usize, usize, usize),
    Chunk(&'a [u8], usize, usize),
}

struct ChunkIterator<'a> {
    data: &'a [u8],
    idx: usize,
}

impl<'a> ChunkIterator<'a> {
    fn repetitions_at(&self, idx: usize) -> usize {
        let val = self.data[idx];
        let mut count = 1;

        while idx + count < self.data.len() && self.data[idx + count] == val {
            count += 1;
        }
        count
    }
}

impl<'a> Iterator for ChunkIterator<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.data.len() {
            return None;
        }

        let reps = self.repetitions_at(self.idx);
        if reps >= 8 {
            // return Repeats chunk
            let chunk = Chunk::Repeats(self.data[self.idx], reps, self.idx, self.idx + reps);
            self.idx += reps;
            Some(chunk)
        } else {
            // find the beginning of the next repeats chunk
            let mut next_rep = self.idx + 1;
            while next_rep < self.data.len() && self.repetitions_at(next_rep) < 8 {
                next_rep += 1;
            }
            let chunk = if next_rep < self.data.len() {
                Chunk::Chunk(&self.data[self.idx..next_rep], self.idx, next_rep)
            } else {
                Chunk::Chunk(&self.data[self.idx..], self.idx, self.data.len())
            };
            self.idx = next_rep;
            Some(chunk)
        }
    }
}

impl<'a> From<&'a [u8]> for ChunkIterator<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self {
            idx: 0,
            data: value,
        }
    }
}

impl Debug for Bin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.0.as_slice();
        write!(
            f,
            "[{}]",
            ChunkIterator::from(data)
                .map(|group| match group {
                    Chunk::Repeats(val, reps, _, _) => {
                        format!("{:#04x} repeats {} times", val, reps)
                    }
                    Chunk::Chunk(ch, _, _) => {
                        format!(
                            "[{}]",
                            ch.iter()
                                .map(|v| format!("{:#04x}", v))
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    }
                })
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Deref for Bin {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chunks_are_as_expected() {
        let data = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4,
        ];
        let chunks = ChunkIterator::from(data.as_slice()).collect::<Vec<_>>();
        let expected = vec![
            Chunk::Repeats(0, 8, 0, 8),
            Chunk::Chunk(&data[8..10], 8, 10),
            Chunk::Repeats(0, 9, 10, 19),
            Chunk::Chunk(&data[19..], 19, 23),
        ];
        assert_eq!(ChunkIterator::from(data.as_slice()).repetitions_at(0), 8);
        assert_eq!(ChunkIterator::from(data.as_slice()).repetitions_at(8), 1);
        assert_eq!(ChunkIterator::from(data.as_slice()).repetitions_at(9), 1);
        assert_eq!(ChunkIterator::from(data.as_slice()).repetitions_at(10), 9);
        assert_eq!(chunks, expected);
    }
}
