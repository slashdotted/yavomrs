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

#[derive(PartialEq, Debug)]
enum OP {
    INSERT,
    DELETE,
    _DELETE,
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Point(i64, i64);

#[derive(Debug)]
pub struct Move<K>(OP, Point, Point, Option<Vec<K>>);

struct Area<'a, K>
where
    K: PartialEq,
{
    m_a: &'a Vec<K>,
    m_b: &'a Vec<K>,
    m_tl: Point,
    m_br: Point,
    m_n: i64,
    m_m: i64,
}

impl<'a, K> Area<'a, K>
where
    K: PartialEq,
{
    pub fn new_from_container(a: &'a Vec<K>, b: &'a Vec<K>) -> Self {
        let mut area = Area {
            m_a: a,
            m_b: b,
            m_tl: Point(0, 0),
            m_br: Point(a.len() as i64, b.len() as i64),
            m_n: 0,
            m_m: 0,
        };
        area.trim();
        area
    }

    pub fn new_from_base(base: &Area<'a, K>, tl: Point, br: Point) -> Self {
        let mut area = Area {
            m_a: base.m_a,
            m_b: base.m_b,
            m_tl: tl,
            m_br: br,
            m_n: 0,
            m_m: 0,
        };
        area.trim();
        area
    }

    fn trim(&mut self) {
        while self.m_tl.0 < self.m_br.0
            && self.m_tl.1 < self.m_br.1
            && self.m_a[self.m_tl.0 as usize] == self.m_b[self.m_tl.1 as usize]
        {
            self.m_tl.0 += 1;
            self.m_tl.1 += 1;
        }
        while self.m_br.0 > self.m_tl.0
            && self.m_br.1 > self.m_tl.1
            && self.m_a[(self.m_br.0 - 1) as usize] == self.m_b[(self.m_br.1 - 1) as usize]
        {
            self.m_br.0 -= 1;
            self.m_br.1 -= 1;
        }
        self.m_n = self.cget_n();
        self.m_m = self.cget_m();
    }

    pub fn a_at(&self, index: i64) -> &K {
        &self.m_a[(self.m_tl.0 + index) as usize]
    }

    pub fn b_at(&self, index: i64) -> &K {
        &self.m_b[(self.m_tl.1 + index) as usize]
    }

    pub fn ra_at(&self, index: i64) -> &K {
        &self.m_a[(self.m_br.0 - 1 - index) as usize]
    }

    pub fn rb_at(&self, index: i64) -> &K {
        &self.m_b[(self.m_br.1 - 1 - index) as usize]
    }

    pub fn get_n(&self) -> i64 {
        self.m_n
    }

    pub fn get_m(&self) -> i64 {
        self.m_m
    }

    pub fn cget_n(&self) -> i64 {
        self.m_br.0 - self.m_tl.0
    }

    pub fn cget_m(&self) -> i64 {
        self.m_br.1 - self.m_tl.1
    }

    pub fn abs_point_r(&self, rel_x: i64, rel_y: i64) -> Point {
        Point(
            self.m_tl.0 + self.get_n() - rel_x,
            self.m_tl.1 + self.get_m() - rel_y,
        )
    }

    pub fn abs_point(&self, rel_x: i64, rel_y: i64) -> Point {
        Point(self.m_tl.0 + rel_x, self.m_tl.1 + rel_y)
    }

    pub fn rdiagonal(&self, k: i64) -> i64 {
        -k + self.get_n() - self.get_m()
    }

    pub fn contains_abs(&self, p: &Point) -> bool {
        p.0 >= self.m_tl.0 && p.0 <= self.m_br.0 && p.1 >= self.m_tl.1 && p.1 <= self.m_br.1
    }

    pub fn tl(&self) -> &Point {
        &self.m_tl
    }

    pub fn br(&self) -> &Point {
        &self.m_br
    }
}

pub fn apply_move<K>(m: &Move<K>, a: &mut Vec<K>)
where
    K: Clone,
{
    let Move(op, s, t, v) = m;
    match op {
        OP::INSERT => {
            a.reserve(v.as_ref().unwrap().len());
            let mut inspoint = a.split_off(s.1 as usize);
            a.extend_from_slice(&v.as_ref().unwrap());
            a.append(&mut inspoint);
        }
        OP::DELETE => {
            let count = t.0 - s.0;
            a.drain((s.1 as usize)..((s.1 + count) as usize));
        }
        OP::_DELETE => {
            let Point(count, start) = s;
            a.drain((*start as usize)..((start + count) as usize));
        }
    }
}

fn myers_middle_move<'a, K>(area: &'a Area<K>) -> (Point, Point)
where
    K: PartialEq,
{
    let max = area.get_m() + area.get_n();
    let mut v_fwd = vec![];
    v_fwd.resize(2 * max as usize + 1, 0);
    let mut x_fwd: i64;
    let mut y_fwd: i64;

    let mut v_bwd = vec![];
    v_bwd.resize(2 * max as usize + 1, 0);
    let mut x_bwd: i64;
    let mut y_bwd: i64;

    let tk = |v: i64| -> usize { (v + max) as usize };

    for d in 0..(max + 1) {
        let min_valid_k: i64 =
            -(d as i64) + std::cmp::max(0, d as i64 - area.get_m() as i64) as i64 * 2;
        let max_valid_k: i64 =
            d as i64 - std::cmp::max(0, d as i64 - area.get_n() as i64) as i64 * 2;
        // Forward step
        let mut at_dest = false;
        let mut px: i64;
        for k in (min_valid_k..(max_valid_k + 1)).step_by(2) {
            // Move downward or to the right
            if k == -(d as i64) || ((k != d) && (v_fwd[tk(k - 1)] < v_fwd[tk(k + 1)])) {
                x_fwd = v_fwd[tk(k + 1)];
                px = x_fwd;
            } else {
                px = v_fwd[tk(k - 1)];
                x_fwd = px + 1;
            }
            y_fwd = x_fwd - k;
            // Follow diagonal as long as possible
            while ((x_fwd < area.get_n()) && (y_fwd < area.get_m()))
                && (area.a_at(x_fwd) == area.b_at(y_fwd))
            {
                x_fwd += 1;
                y_fwd += 1;
            }
            // Store best x position on this diagonal
            v_fwd[tk(k)] = x_fwd;

            // Check if we crossed the backward move
            if d > 0 {
                let rk = area.rdiagonal(k);
                if x_fwd >= (area.get_n() - v_bwd[tk(rk)]) {
                    let top = area.abs_point(px, px - k);
                    if area.contains_abs(&top) {
                        let bottom = area.abs_point(x_fwd, y_fwd);
                        if area.contains_abs(&bottom) {
                            return (top, bottom);
                        }
                    }
                }
            }

            if x_fwd >= area.get_n() && y_fwd >= area.get_m() {
                at_dest = true;
                break;
            }
        }

        // Backward step
        for k in (min_valid_k..(max_valid_k + 1)).step_by(2) {
            // Move downward or to the right
            if k == -(d as i64) || ((k != d) && (v_bwd[tk(k - 1)] < v_bwd[tk(k + 1)])) {
                x_bwd = v_bwd[tk(k + 1)];
                px = x_bwd;
            } else {
                px = v_bwd[tk(k - 1)];
                x_bwd = px + 1;
            }
            y_bwd = x_bwd - k;
            // Follow diagonal as long as possible
            while ((x_bwd < area.get_n()) && (y_bwd < area.get_m()))
                && (area.ra_at(x_bwd) == area.rb_at(y_bwd))
            {
                x_bwd += 1;
                y_bwd += 1;
            }
            // Store best x position on this diagonal
            v_bwd[tk(k)] = x_bwd;

            // Check if we crossed the forward move
            if d > 0 {
                let rk = area.rdiagonal(k);
                if x_bwd >= (area.get_n() - v_fwd[tk(rk)]) {
                    let top = area.abs_point_r(x_bwd, y_bwd);
                    if area.contains_abs(&top) {
                        let bottom = area.abs_point_r(px, px - k);
                        if area.contains_abs(&bottom) {
                            return (top, bottom);
                        }
                    }
                }
            }

            if x_bwd >= area.get_n() && y_bwd >= area.get_m() {
                at_dest = true;
                break;
            }
        }

        if at_dest {
            break;
        }
    }
    panic!("This can't be")
}

fn myers_moves<'a, K>(area: &'a Area<K>, result: &mut Vec<Move<K>>)
where
    K: PartialEq,
{
    if area.get_n() == 0 && area.get_m() == 0 {
        return;
    } else if area.get_n() == 0 {
        if !result.is_empty() {
            let last = result.last_mut().unwrap();
            // Merge with last insert (if possible)
            if last.0 == OP::INSERT && last.2 == *area.tl() {
                last.2 = *area.br();
                return;
            }
        }
        result.push(Move(OP::INSERT, *area.tl(), *area.br(), None));
    } else if area.get_m() == 0 {
        if !result.is_empty() {
            let last = result.last_mut().unwrap();
            // Merge with last delete (if possible)
            if last.0 == OP::DELETE && last.2 == *area.tl() {
                last.2 = *area.br();
                return;
            }
        }
        result.push(Move(OP::DELETE, *area.tl(), *area.br(), None));
    } else {
        let (top, bottom) = myers_middle_move(area);
        myers_moves(&Area::new_from_base(&area, *area.tl(), top), result);
        myers_moves(&Area::new_from_base(&area, top, bottom), result);
        myers_moves(&Area::new_from_base(&area, bottom, *area.br()), result)
    }
}

pub fn myers<K>(a: &Vec<K>, b: &Vec<K>) -> Vec<Move<K>>
where
    K: PartialEq + Clone,
{
    let mut s = myers_unfilled(a, b);
    myers_fill(b, &mut s);
    s
}

pub fn myers_unfilled<K>(a: &Vec<K>, b: &Vec<K>) -> Vec<Move<K>>
where
    K: PartialEq,
{
    let all = Area::new_from_container(a, b);
    let mut s = vec![];
    myers_moves(&all, &mut s);
    s
}

pub fn myers_fill<K>(b: &Vec<K>, s: &mut Vec<Move<K>>)
where
    K: PartialEq + Clone,
{
    s.iter_mut().for_each(|m| {
        let Move(op, s, t, v) = m;
        match op {
            OP::INSERT => {
                let count = t.1 - s.1;
                let from = s.1 as usize;
                let to = (s.1 + count) as usize;
                let vc = v.get_or_insert_with(|| Vec::new());
                vc.extend_from_slice(&b[from..to])
            }
            OP::DELETE => {}
            OP::_DELETE => {}
        }
    })
}

pub fn myers_strip_moves<K>(s: &mut Vec<Move<K>>)
where
    K: PartialEq + Clone,
{
    s.iter_mut().for_each(|m| {
        let Move(op, s, t, _) = m;
        match op {
            OP::DELETE => {
                *op = OP::_DELETE;
                let count = t.0 - s.0;
                *s = Point(count, s.1);
            }
            OP::INSERT => {}
            OP::_DELETE => {}
        }
    })
}

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
        let mut a: Vec<String> = vec!["A", "W", "E", "S", "O", "M", "O"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let b: Vec<String> = vec!["S", "T", "R", "A", "N", "G", "E", "S", "O", "M", "O"]
            .iter()
            .map(|s| s.to_string())
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
