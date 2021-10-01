use super::super::manager;
//use super::source;
use super::token;

pub struct Content {
  pub seq: isize,
  pub ty: isize,
  pub is_dummy: bool,
  pub is_cur: bool,
  pub page: isize,
  pub index: isize,
  pub label: Option<String>,
  pub tokens: Vec<token::Token>,
  pub children: Vec<Content>,
}

pub struct ContentsTree {
  pub root: Option<Content>,
}

impl Content {
  pub fn new(seq: isize, ty: isize, is_dummy: bool) -> Self {
    let c = Content {
      seq,
      ty,
      is_dummy,
      is_cur: false,
      page: 0,
      index: 0,
      label: None,
      tokens: Vec::new(),
      children: Vec::new(),
    };

    c
  }
}

impl ContentsTree {
  pub fn new(mgr: &manager::Manager) -> Self {
    let mut t = ContentsTree { root: None };
    let root = t.build(mgr);
    t.root = Some(root);

    t
  }

  fn build(&mut self, mgr: &manager::Manager) -> Content {
    log!("***ContentsTree.build");
    let (_, _, root) = self.build_sub(mgr, 0, 0, true);

    root
  }

  fn build_sub(
    &mut self,
    mgr: &manager::Manager,
    idx: usize,
    lvl: isize,
    is_dummy: bool,
  ) -> (usize, bool, Content) {
    log!("***ContentsTree.build_sub: idx={} lvl={}", idx, lvl);
    let mut con = Content::new(idx as isize, lvl, is_dummy);
    let lv = lvl + 1;
    let mut i = idx;
    let mut is_cur = false;

    loop {
      if i >= mgr.sources.len() {
        break;
      }

      let s = &mgr.sources[i];

      if s.ty == 0 {
        if s.tokens.len() == 0 {
          i += 1;
          continue;
        }

        let mut c = Content::new(s.seq, s.ty, false);

        for t in &s.tokens {
          if c.label.is_none() {
            match t.ty {
              token::TokenType::Zenkaku | token::TokenType::Kana | token::TokenType::Alpha => {
                for ch in t.word.chars() {
                  let l = format!("{}", ch);
                  c.label = Some(l.clone());
                  if con.label.is_none() {
                    con.label = Some(l);
                  }
                  break;
                }
              }
              _ => {}
            }
          }
          c.tokens.push(t.clone());
        }

        con.children.push(c);
        i += 1;
        continue;
      }

      if s.ty < lv {
        break;
      }

      let dm: bool;
      if s.ty == lv {
        i += 1;
        dm = false;
      } else {
        dm = true;
      }

      let (index, current, content) = self.build_sub(mgr, i, lv, dm);

      i = index;
      if s.seq == mgr.section {
        is_cur = true;
        con.is_cur = true;
      } else {
        is_cur = current;
        con.is_cur = current;
      }

      if con.label.is_none() && content.label.is_some() {
        let l = content.label.clone().unwrap();
        con.label = Some(l);
      }

      con.children.push(content);
    }

    (i, is_cur, con)
  }

  /*
  fn build(&mut self, mgr: &manager::Manager) {
    let mut i: usize = 0;
    let mut pg1 = 0;
    let mut id1 = 0;

    loop {
      if i >= mgr.sources.len() {
        break;
      }

      let s1 = &mgr.sources[i];

      if s1.ty > 0 {
        let mut c1: Content;

        if s1.ty == 1 {
          c1 = Content::new(s1.seq, s1.ty, false);
          i += 1;
        } else {
          c1 = Content::new(s1.seq, s1.ty, true);
        }

        if s1.seq == mgr.section {
          c1.is_cur = true;
        }

        let mut pg2 = 0;
        let mut id2 = 0;

        loop {
          if i >= mgr.sources.len() {
            break;
          }

          let s2 = &mgr.sources[i];

          if s2.ty > 0 {
            let mut c2: Content;

            if s2.ty == 2 {
              c2 = Content::new(s2.seq, s2.ty, false);
              i += 1;
            } else if s2.ty > 2 {
              c2 = Content::new(s2.seq, s2.ty, true);
            } else {
              break;
            }

            if s2.seq == mgr.section {
              c1.is_cur = true;
              c2.is_cur = true;
            }

            let mut pg3 = 0;
            let mut id3 = 0;

            loop {
              if i >= mgr.sources.len() {
                break;
              }

              let s3 = &mgr.sources[i];

              if s3.ty > 0 {
                let mut c3: Content;

                if s3.ty == 3 {
                  c3 = Content::new(s3.seq, s3.ty, false);
                  i += 1;
                } else if s3.ty > 3 {
                  c3 = Content::new(s3.seq, s3.ty, true);
                } else {
                  break;
                }

                if s3.seq == mgr.section {
                  c1.is_cur = true;
                  c2.is_cur = true;
                  c3.is_cur = true;
                }

                let mut pg4 = 0;
                let mut id4 = 0;

                loop {
                  if i >= mgr.sources.len() {
                    break;
                  }

                  let s4 = &mgr.sources[i];

                  if s4.ty > 0 {
                    let mut c4: Content;

                    if s4.ty == 4 {
                      c4 = Content::new(s4.seq, s4.ty, false);
                      i += 1;
                    } else if s4.ty > 4 {
                      c4 = Content::new(s4.seq, s4.ty, true);
                    } else {
                      break;
                    }

                    if s4.seq == mgr.section {
                      c1.is_cur = true;
                      c2.is_cur = true;
                      c3.is_cur = true;
                      c4.is_cur = true;
                    }

                    let mut pg5 = 0;
                    let mut id5 = 0;

                    loop {
                      if i >= mgr.sources.len() {
                        break;
                      }

                      let s5 = &mgr.sources[i];

                      if s5.ty > 0 {
                        let mut c5: Content;

                        if s5.ty == 5 {
                          c5 = Content::new(s5.seq, s5.ty, false);
                          i += 1;
                        } else if s5.ty > 5 {
                          c5 = Content::new(s5.seq, s5.ty, true);
                        } else {
                          break;
                        }

                        if s5.seq == mgr.section {
                          c1.is_cur = true;
                          c2.is_cur = true;
                          c3.is_cur = true;
                          c4.is_cur = true;
                          c5.is_cur = true;
                        }

                        let mut pg6 = 0;
                        let mut id6 = 0;

                        loop {
                          if i >= mgr.sources.len() {
                            break;
                          }

                          let s6 = &mgr.sources[i];

                          if s6.ty > 0 {
                            let mut c6: Content;

                            if s6.ty == 6 {
                              c6 = Content::new(s6.seq, s6.ty, false);
                              i += 1;
                            } else {
                              break;
                            }

                            if s6.seq == mgr.section {
                              c1.is_cur = true;
                              c2.is_cur = true;
                              c3.is_cur = true;
                              c4.is_cur = true;
                              c5.is_cur = true;
                              c6.is_cur = true;
                            }

                            c6.page = pg6;
                            c6.index = id6;
                            c5.children.push(c6);
                          } else {
                            let mut c6 = Content::new(s6.seq, s6.ty, false);
                            c6.page = pg6;
                            c6.index = id6;
                            c5.children.push(c6);
                            i += 1;
                          }

                          id6 += 1;
                          if id6 > 8 {
                            id6 = 0;
                            pg6 += 1;
                          }
                        }

                        c5.page = pg5;
                        c5.index = id5;
                        c4.children.push(c5);
                      } else {
                        let mut c5 = Content::new(s5.seq, s5.ty, false);
                        c5.page = pg5;
                        c5.index = id5;
                        c4.children.push(c5);
                        i += 1;
                      }

                      id5 += 1;
                      if id5 > 8 {
                        id5 = 0;
                        pg5 += 1;
                      }
                    }

                    c4.page = pg4;
                    c4.index = id4;
                    c3.children.push(c4);
                  } else {
                    let mut c4 = Content::new(s4.seq, s4.ty, false);
                    c4.page = pg4;
                    c4.index = id4;
                    c3.children.push(c4);
                    i += 1;
                  }

                  id4 += 1;
                  if id4 > 8 {
                    id4 = 0;
                    pg4 += 1;
                  }
                }

                c3.page = pg3;
                c3.index = id3;
                c2.children.push(c3);
              } else {
                let mut c3 = Content::new(s3.seq, s3.ty, false);
                c3.page = pg3;
                c3.index = id3;
                c2.children.push(c3);
                i += 1;
              }

              id3 += 1;
              if id3 > 8 {
                id3 = 0;
                pg3 += 1;
              }
            }

            c2.page = pg2;
            c2.index = id2;
            c1.children.push(c2);
          } else {
            let mut c2 = Content::new(s2.seq, s2.ty, false);
            c2.page = pg2;
            c2.index = id2;
            c1.children.push(c2);
            i += 1;
          }

          id2 += 1;
          if id2 > 8 {
            id2 = 0;
            pg2 += 1;
          }
        }

        c1.page = pg1;
        c1.index = id1;
        self.root.children.push(c1);
      } else {
        let mut c1 = Content::new(s1.seq, s1.ty, false);
        c1.page = pg1;
        c1.index = id1;
        self.root.children.push(c1);
        i += 1;
      }

      id1 += 1;
      if id1 > 8 {
        id1 = 0;
        pg1 += 1;
      }
    }
  }
  */

  /*
  fn build(&mut self, mgr: &manager::Manager) {
    let mut cons: Vec<isize> = Vec::new();
    let top = Content::new(-1, 1);
    self.root.children.push(top);
    cons.push(-2);
    cons.push(-1);
    let mut c: usize;

    for s in &mgr.sources {
      c = cons.len() - 1;
      //if let Some(c1) = &cons[c] {
      //let mut c1 = mgr.sources[cons[c] as usize];
      let mut c1 = cons[c];
      if c1 == -1 {
        if s.ty > 0 {
          let c2 = Content::new(s.seq, s.ty);
          self.root.children.push(c2);
        } else {
          let c2 = Content::new(s.seq, s.ty);
          let mut top = self.root.children.pop().unwrap();
          top.children.push(c2);
          self.root.children.push(top);
        }
      } else {
        let s1 = mgr.sources[c1 as usize];
        if s.ty > 0 {
          if s1.ty == s.ty {
            let c2 = Content::new(s.seq, s.ty);
            self.root.children.push(c2);
            //c1 = c2;
            let mut i = c + 1;
            loop {
              if cons.len() < c {
                break;
              }
              cons.pop();
            }
          } else if c1.ty < s.ty {
            let c2 = Content::new(s.seq, s.ty);
            c1.children.push(c2);
            c += 1;
            cons.push(c2);
            //let mut i = c + 1;
            loop {
              if cons.len() < c {
                break;
              }
              cons.pop();
            }
          } else {
            loop {
              if cons.len() == 0 {
                break;
              }
              cons.pop();
              c = cons.len() - 1;
              if cons[c].ty == s.ty {
                cons.pop();
                c = cons.len() - 1;
                let c2 = Content::new(s.seq, s.ty);
                cons[c].children.push(c2);
                c += 1;
                cons.push(c2);
                break;
              }
            }
          }

          if s.seq == mgr.section {
            let mut i = 0;
            loop {
              if i >= cons.len() {
                break;
              }
              cons[i].is_cur = true;
              i += 1;
            }
          }
        } else {
          let tx = Content::new(s.seq, s.ty);
          c1.children.push(tx);
          cons.push(tx);
        }
      }
    }
  }
  */

  /*
  fn build(&mut self, mgr: &manager::Manager) {
    let mut cons: Vec<Content> = Vec::new();
    let top = Content::new(-1, 1);
    self.root.children.push(top);
    //let mut seq = -1;

    cons.push(self.root); // root
    cons.push(top); // 見出し1
    let mut c: usize;

    for s in &mgr.sources {
      c = cons.len() - 1;
      //if let Some(c1) = &cons[c] {
      let mut c1 = cons[c];
      if s.ty > 0 {
        if c1.ty == s.ty {
          let mut c2 = Content::new(s.seq, s.ty);
          self.root.children.push(c2);
          c1 = c2;
          let mut i = c + 1;
          loop {
            if cons.len() < c {
              break;
            }
            cons.pop();
          }
        } else if c1.ty < s.ty {
          let c2 = Content::new(s.seq, s.ty);
          c1.children.push(c2);
          c += 1;
          cons.push(c2);
          //let mut i = c + 1;
          loop {
            if cons.len() < c {
              break;
            }
            cons.pop();
          }
        } else {
          loop {
            if cons.len() == 0 {
              break;
            }
            cons.pop();
            c = cons.len() - 1;
            if cons[c].ty == s.ty {
              cons.pop();
              c = cons.len() - 1;
              let c2 = Content::new(s.seq, s.ty);
              cons[c].children.push(c2);
              c += 1;
              cons.push(c2);
              break;
            }
          }
        }

        if s.seq == mgr.section {
          let mut i = 0;
          loop {
            if i >= cons.len() {
              break;
            }
            cons[i].is_cur = true;
            i += 1;
          }
        }
      } else {
        let tx = Content::new(s.seq, s.ty);
        c1.children.push(tx);
        cons.push(tx);
      }
      //}
    }
  }
  */

  /*
  fn build(&mut self, mgr: &manager::Manager) {
    let mut cons: Vec<Option<Content>> = Vec::new();
    let top = Content::new(-1, 1);
    self.root.children.push(top);
    //let mut seq = -1;

    cons.push(Some(self.root)); // root
    cons.push(Some(top)); // 見出し1
    cons.push(None); // 見出し2
    cons.push(None); // 見出し3
    cons.push(None); // 見出し4
    cons.push(None); // 見出し5
    cons.push(None); // 見出し6
    let mut c: usize = 1;

    for s in &mgr.sources {
      if let Some(c1) = &cons[c] {
        if s.ty > 0 {
          if c1.ty == s.ty {
            let mut c2 = Content::new(s.seq, s.ty);
            self.root.children.push(c2);
            cons[c] = Some(c2);
            let mut i = c + 1;
            loop {
              if i >= cons.len() {
                break;
              }
              cons[i] = None;
              i += 1;
            }
          } else if c1.ty < s.ty {
            let c2 = Content::new(s.seq, s.ty);
            c1.children.push(c2);
            c += 1;
            cons[c] = Some(c2);
            let mut i = c + 1;
            loop {
              if i >= cons.len() {
                break;
              }
              cons[i] = None;
              i += 1;
            }
          } else {
            let mut t = c;
            loop {
              t -= 1;
              if t < 1 {
                break;
              }
              if let Some(c2) = &cons[t] {
                if c2.ty == s.ty {
                  if let Some(c3) = &cons[t - 1] {
                    let c4 = Content::new(s.seq, s.ty);
                    c3.children.push(c4);
                    c = t;
                    cons[c] = Some(c4);
                    let mut i = c + 1;
                    loop {
                      if i >= cons.len() {
                        break;
                      }
                      cons[i] = None;
                      i += 1;
                    }
                  }
                  break;
                }
              }
            }
          }

          if s.seq == mgr.section {
            let mut i = c;
            loop {
              if i < 0 {
                break;
              }
              if let Some(cx) = &cons[i] {
                cx.is_cur = true;
              }
              i -= 1;
            }
          }
        } else {
          let tx = Content::new(s.seq, s.ty);
          c1.children.push(tx);
          cons[c + 1] = Some(tx);
        }
      }
    }
  }
  */
}
