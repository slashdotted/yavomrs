/* Copyright 2022 Amos Brocco - contact@amosbrocco.ch
 *
 * Redistribution and use in source and binary forms, with or without modification,
 * are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * 3. Neither the name of the copyright holder nor the names of its contributors
 *    may be used to endorse or promote products derived from this software without
 *    specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
 * ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

pub mod yavom;


mod tests {
    use std::io::BufRead;

    #[allow(dead_code)]
    fn read_lines<P>(
        filename: P,
    ) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
    where
        P: AsRef<std::path::Path>,
    {
        let file = std::fs::File::open(filename)?;
        Ok(std::io::BufReader::new(file).lines())
    }

    #[test]
    pub fn test_simple() {
        eprint!("Comparing simple strings...");
        let mut a: Vec<String> = vec!["A", "W", "E", "S", "O", "M", "O"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let b: Vec<String> = vec!["S", "T", "R", "A", "N", "G", "E", "S", "O", "M", "O"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let moves = crate::yavom::myers(&a, &b);
        eprint!("{} moves...", moves.len());
        moves.iter().for_each(|m| {
            crate::yavom::apply_move(m, &mut a);
        });
        if a != b {
            eprintln!(" fail!");
        } else {
            eprintln!(" success!");
        }
    }

    #[test]
    pub fn test_huge() {
        for s in 3..24u32 {
            let asize = 2i32.pow(s);
            eprint!("Comparing arrays of size {}...", asize);
            let mut a: Vec<i64> = Vec::with_capacity(asize as usize);
            for x in 0..a.capacity() {
                a.push(x as i64);
            };
            let mut b = a.clone();
            let v = vec![-1,-5,-6];
            b.reserve(v.len());
            let mut inspoint = b.split_off((asize/2) as usize);
            b.extend_from_slice(&v);
            b.append(&mut inspoint);
            b.drain(b.len()-3.. b.len()-1);
            let moves = crate::yavom::myers(&a, &b);
            eprint!("{} moves...", moves.len());
            moves.iter().for_each(|m| {
                crate::yavom::apply_move(m, &mut a);
            });
            if a != b {
                eprintln!(" fail!");
            } else {
                eprintln!(" success!");
            }
        }
    }

    #[test]
    pub fn test_myers_unfilled_strip() {
        let base_path = std::path::Path::new("./testdata");
        let files = [
            "alpha", "ban", "ben", "beta", "delta", "empty", "first", "gamma", "huge", "huge2",
            "large1", "large2", "second", "test1", "test2", "third", "x", "y",
        ];
        for fa in files {
            for fb in files {
                eprint!("Comparing (two steps) {} with {}...", fa, fb);
                let mut a: Vec<String> = read_lines(base_path.join(fa))
                    .unwrap()
                    .map(|v| v.unwrap())
                    .collect();
                let b: Vec<String> = read_lines(base_path.join(fb))
                    .unwrap()
                    .map(|v| v.unwrap())
                    .collect();
                let mut moves = crate::yavom::myers_unfilled(&a, &b);
                eprint!("{} moves...", moves.len());
                crate::yavom::myers_fill(&b, &mut moves);
                crate::yavom::myers_strip_moves(&mut moves);
                eprint!(" filled...");
                moves.iter().for_each(|m| {
                    crate::yavom::apply_move(m, &mut a);
                });
                if a != b {
                    eprintln!(" fail!");
                } else {
                    eprintln!(" success!");
                }
            }
        }
    }

    #[test]
    pub fn test_myers_unfilled() {
        let base_path = std::path::Path::new("./testdata");
        let files = [
            "alpha", "ban", "ben", "beta", "delta", "empty", "first", "gamma", "huge", "huge2",
            "large1", "large2", "second", "test1", "test2", "third", "x", "y",
        ];
        for fa in files {
            for fb in files {
                eprint!("Comparing (two steps) {} with {}...", fa, fb);
                let mut a: Vec<String> = read_lines(base_path.join(fa))
                    .unwrap()
                    .map(|v| v.unwrap())
                    .collect();
                let b: Vec<String> = read_lines(base_path.join(fb))
                    .unwrap()
                    .map(|v| v.unwrap())
                    .collect();
                let mut moves = crate::yavom::myers_unfilled(&a, &b);
                eprint!("{} moves...", moves.len());
                crate::yavom::myers_fill(&b, &mut moves);
                eprint!(" filled...");
                moves.iter().for_each(|m| {
                    crate::yavom::apply_move(m, &mut a);
                });
                if a != b {
                    eprintln!(" fail!");
                } else {
                    eprintln!(" success!");
                }
            }
        }
    }

    #[test]
    pub fn test_myers() {
        let base_path = std::path::Path::new("./testdata");
        let files = [
            "alpha", "ban", "ben", "beta", "delta", "empty", "first", "gamma", "huge", "huge2",
            "large1", "large2", "second", "test1", "test2", "third", "x", "y",
        ];
        for fa in files {
            for fb in files {
                eprint!("Comparing (two steps) {} with {}...", fa, fb);
                let mut a: Vec<String> = read_lines(base_path.join(fa))
                    .unwrap()
                    .map(|v| v.unwrap())
                    .collect();
                let b: Vec<String> = read_lines(base_path.join(fb))
                    .unwrap()
                    .map(|v| v.unwrap())
                    .collect();
                let moves = crate::yavom::myers(&a, &b);
                moves.iter().for_each(|m| {
                    crate::yavom::apply_move(m, &mut a);
                });
                if a != b {
                    eprintln!(" fail!");
                } else {
                    eprintln!(" success!");
                }
            }
        }
    }
}