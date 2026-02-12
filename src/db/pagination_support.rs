use sqlx::{QueryBuilder, Sqlite};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PaginationDirection {
    Forward,
    Backward,
}

pub fn keyset_paginate(
    limit: u32,
    cursor: Option<u32>,
    direction: PaginationDirection,
    qb: &mut QueryBuilder<Sqlite>,
) {
    match direction {
        PaginationDirection::Forward => {
            if let Some(last_id) = cursor {
                qb.push(" AND id > ");
                qb.push_bind(last_id);
            }
            qb.push(" ORDER BY id ASC LIMIT ");
        }
        PaginationDirection::Backward => {
            if let Some(first_id) = cursor {
                qb.push(" AND id < ");
                qb.push_bind(first_id);
            }
            qb.push(" ORDER BY id DESC LIMIT ");
        }
    }
    qb.push_bind(limit + 1);
}

pub trait HasId {
    fn id(&self) -> u32;
}

pub struct NextAndPrevCursor {
    pub next_cursor: Option<u32>,
    pub prev_cursor: Option<u32>,
}

pub struct PaginationRes<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<u32>,
    pub prev_cursor: Option<u32>,
}

pub fn get_cursors<T: HasId>(
    limit: u32,
    cursor: Option<u32>,
    direction: PaginationDirection,
    rows: &mut Vec<T>,
) -> NextAndPrevCursor {
    let has_more = rows.len() > limit as usize;
    if has_more {
        rows.pop();
    }

    if matches!(direction, PaginationDirection::Backward) {
        rows.reverse();
    }

    let start_id = rows.first().map(|r| r.id());
    let end_id = rows.last().map(|r| r.id());

    let (next_cursor, prev_cursor) = match direction {
        PaginationDirection::Forward => {
            let next = if has_more { end_id } else { None };
            let prev = if cursor.is_some() { start_id } else { None };
            (next, prev)
        }
        PaginationDirection::Backward => {
            let next = end_id;
            let prev = if has_more { start_id } else { None };
            (next, prev)
        }
    };

    NextAndPrevCursor {
        next_cursor,
        prev_cursor,
    }
}
