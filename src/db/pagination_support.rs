use sqlx::{QueryBuilder, Sqlite};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PaginationDirection {
    Forward,
    Backward,
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

pub struct PaginationParams {
    pub limit: u32,
    pub cursor: Option<u32>,
    pub direction: PaginationDirection,
}

pub struct PaginationState {
    pub limit: u32,
    pub next_cursor: Option<u32>,
    pub prev_cursor: Option<u32>,
    pub current_cursor: Option<u32>, // Cursor used for current page
    pub direction: PaginationDirection,
}

impl Default for PaginationState {
    fn default() -> Self {
        Self {
            limit: 50,
            next_cursor: None,
            prev_cursor: None,
            current_cursor: None,
            direction: PaginationDirection::Forward,
        }
    }
}

impl PaginationState {
    pub fn reset_pagination(&mut self) {
        self.next_cursor = None;
    }

    pub fn has_previous(&self) -> bool {
        self.prev_cursor.is_some()
    }

    pub fn has_next(&self) -> bool {
        self.next_cursor.is_some()
    }

    pub fn go_backwards(&mut self) {
        self.current_cursor = self.prev_cursor;
        self.direction = PaginationDirection::Backward;
    }

    pub fn go_forwards(&mut self) {
        self.current_cursor = self.next_cursor;
        self.direction = PaginationDirection::Forward;
    }

    pub fn to_pagination_params(&self) -> PaginationParams {
        PaginationParams {
            limit: self.limit,
            cursor: self.current_cursor,
            direction: self.direction,
        }
    }
}

pub fn keyset_paginate(params: &PaginationParams, qb: &mut QueryBuilder<Sqlite>) {
    match params.direction {
        PaginationDirection::Forward => {
            if let Some(last_id) = params.cursor {
                qb.push(" AND id > ");
                qb.push_bind(last_id);
            }
            qb.push(" ORDER BY id ASC LIMIT ");
        }
        PaginationDirection::Backward => {
            if let Some(first_id) = params.cursor {
                qb.push(" AND id < ");
                qb.push_bind(first_id);
            }
            qb.push(" ORDER BY id DESC LIMIT ");
        }
    }
    qb.push_bind(params.limit + 1);
}

pub fn get_cursors<T: HasId>(params: &PaginationParams, rows: &mut Vec<T>) -> NextAndPrevCursor {
    let has_more = rows.len() > params.limit as usize;
    if has_more {
        rows.pop();
    }

    if matches!(params.direction, PaginationDirection::Backward) {
        rows.reverse();
    }

    let start_id = rows.first().map(|r| r.id());
    let end_id = rows.last().map(|r| r.id());

    let (next_cursor, prev_cursor) = match params.direction {
        PaginationDirection::Forward => {
            let next = if has_more { end_id } else { None };
            let prev = if params.cursor.is_some() {
                start_id
            } else {
                None
            };
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
