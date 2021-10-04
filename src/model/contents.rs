use super::super::manager;
use super::token;

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone)]
pub enum ContentType {
  Content,
  Section,
  Text,
}

pub struct ContentTree {
  pub ty: ContentType,
  pub seq: isize,
  pub lbl: isize,
  pub is_dummy: bool,
  pub is_cur: bool,
  pub page: isize,
  pub label: Option<String>,
  pub token2s: Vec<token::Token2>,
  pub children: Vec<ContentTree>,
}

impl ContentTree {
  pub fn new(ty: ContentType, seq: isize, lbl: isize, is_dummy: bool) -> Self {
    let c = ContentTree {
      ty,
      seq,
      lbl,
      is_dummy,
      is_cur: false,
      page: 0,
      label: None,
      token2s: Vec::new(),
      children: Vec::new(),
    };

    c
  }

  pub fn build(mgr: &manager::Manager) -> Self {
    let mut c = ContentTree {
      ty: ContentType::Content,
      seq: 0,
      lbl: 0,
      is_dummy: true,
      is_cur: false,
      page: 0,
      label: None,
      token2s: Vec::new(),
      children: Vec::new(),
    };
    let (_, root) = c.build_sub(mgr, 0, 0, 0, false, true);

    root
  }

  fn build_sub(
    &mut self,
    mgr: &manager::Manager,
    idx: usize,
    seq: isize,
    lvl: isize,
    is_cur: bool,
    is_dummy: bool,
  ) -> (usize, ContentTree) {
    /*
    log!(
      "***PanelBox.build_sub: idx={} seq={} lvl={} is_cur={} mgr.section={}",
      idx,
      seq,
      lvl,
      is_cur,
      mgr.section
    );
    */

    let mut con = ContentTree::new(ContentType::Content, idx as isize, lvl, is_dummy);
    let lv = lvl + 1;
    let mut i = idx;
    //let mut is_con = true;

    if mgr.section == -1 {
      if seq == 0 {
        con.is_cur = true;
      }
    } else if seq == mgr.section {
      con.is_cur = true;
    }

    loop {
      if i >= mgr.sources.len() {
        break;
      }

      let s = &mgr.sources[i];

      if s.ty == 0 {
        let mut section = ContentTree::new(ContentType::Section, s.seq, lv, false);
        //let mut j = i;
        loop {
          if i >= mgr.sources.len() {
            break;
          }
          let s2 = &mgr.sources[i];
          if s2.ty != 0 {
            break;
          }
          let mut c = ContentTree::new(ContentType::Text, s2.seq, lv + 1, false);

          for t in &s2.token2s {
            if c.label.is_none() {
              match t.ty {
                token::TokenType::Zenkaku | token::TokenType::Kana | token::TokenType::Alpha => {
                  for ch in t.word.chars() {
                    let l = format!("{}", ch);
                    c.label = Some(l.clone());
                    if section.label.is_none() {
                      section.label = Some(l);
                    }
                    break;
                  }
                }
                _ => {}
              }
            }
            c.token2s.push(t.clone());
          }
          section.children.push(c);
          i += 1;
        }

        if con.label.is_none() && section.label.is_some() {
          con.label = Some(section.label.clone().unwrap());
        }
        if con.is_cur && con.children.len() == 0 {
          section.is_cur = true;
        }
        con.children.push(section);
        continue;
      }

      if s.ty < lv {
        break;
      }

      //is_con = true;
      let dm: bool;
      if s.ty == lv {
        i += 1;
        dm = false;
      } else {
        dm = true;
      }

      let (index, mut content) = self.build_sub(mgr, i, s.seq, lv, con.is_cur, dm);

      i = index;
      if content.is_cur {
        con.is_cur = true;
      }

      if con.label.is_none() && content.label.is_some() {
        let l = content.label.clone().unwrap();
        con.label = Some(l);
      }

      if is_cur && con.children.len() == 0 {
        content.is_cur = true;
      }

      con.children.push(content);
    }

    (i, con)
  }

  pub fn get_cur_sec(&self) -> Option<&ContentTree> {
    for c in &self.children {
      match c.ty {
        ContentType::Content => {
          if c.is_cur {
            return c.get_cur_sec();
          }
        }
        ContentType::Section => {
          if c.is_cur {
            return Some(c);
          }
        }
        ContentType::Text => {}
      }
    }

    None
  }
}
