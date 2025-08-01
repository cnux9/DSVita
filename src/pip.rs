use crate::presenter::{PresenterScreen, PRESENTER_SCREEN_WIDTH, PRESENTER_SCREEN_HEIGHT, PRESENTER_SUB_PIP_SCREEN_WIDTH_BOT, PRESENTER_SUB_PIP_SCREEN_HEIGHT_BOT, PRESENTER_SUB_PIP_SCREEN_WIDTH_TOP, GAP};

#[derive(Clone, Copy, Default)]
struct Pos {
    x: i32,
    y: i32,
}

const SPEED:     i32 = 3;
const SNAP_MIN:  i32 = 2;

pub const AX: u32 = PRESENTER_SCREEN_WIDTH - PRESENTER_SUB_PIP_SCREEN_WIDTH_BOT - GAP;
pub const AY: u32 = PRESENTER_SCREEN_HEIGHT - PRESENTER_SUB_PIP_SCREEN_HEIGHT_BOT - GAP;
pub const BX: u32 = PRESENTER_SUB_PIP_SCREEN_WIDTH_TOP + GAP;
pub const BY: u32 = (PRESENTER_SCREEN_HEIGHT - PRESENTER_SUB_PIP_SCREEN_HEIGHT_BOT) * 2 / 3;
pub const CX: u32 = PRESENTER_SCREEN_WIDTH - PRESENTER_SUB_PIP_SCREEN_WIDTH_BOT - GAP;
pub const CY: u32 = GAP;
pub const SNAP_POINTS: [(i32, i32); 3] = [(AX as i32, AY as i32), (BX as i32, BY as i32), (CX as i32, CY as i32)];

static mut CUR:       Pos           = Pos { x: 0, y: 0 };
static mut TARGET:    Pos           = Pos { x: 0, y: 0 };
static mut START_TCH: Option<(i32,i32)> = None;
static mut START_TGT: Pos           = Pos { x: 0, y: 0 };

pub fn update_touch(cx: i32, cy: i32, touching: bool) {
    unsafe {
        if touching {
            match START_TCH {
                None => {
                    START_TCH = Some((cx, cy));
                    START_TGT = TARGET;
                }
                Some((sx, sy)) => {
                    TARGET.x = (cx - sx) * SPEED + START_TGT.x;
                    TARGET.y = (cy - sy) * SPEED + START_TGT.y;
                }
            }
        } else if START_TCH.is_some() {
            START_TCH = None;

            let (sx, sy) = SNAP_POINTS
                .iter()
                .copied()
                .min_by_key(|(px, py)| {
                    let dx = TARGET.x - px;
                    let dy = TARGET.y - py;
                    dx*dx + dy*dy
                })
                .unwrap();

            TARGET.x = sx;
            TARGET.y = sy;
        }
    }
}

pub fn tick() {
    unsafe {
        let step = |cur: &mut i32, tgt: i32| {
            let d = tgt - *cur;
            if d.abs() <= SNAP_MIN {
                *cur = tgt;
            } else {
                *cur += d / 2;
            }
        };
        step(&mut CUR.x, TARGET.x);
        step(&mut CUR.y, TARGET.y);
    }
}

pub fn presenter_screen() -> PresenterScreen {
    unsafe {
        PresenterScreen::new(
            CUR.x as u32,
            CUR.y as u32,
            PRESENTER_SUB_PIP_SCREEN_WIDTH_BOT  as u32,
            PRESENTER_SUB_PIP_SCREEN_HEIGHT_BOT as u32,
        )
    }
}