use {bitflags::bitflags, tui::layout::Rect};

bitflags! {
    pub struct Edges: u32 {

        const LEFT      = 0b00000001;
        const RIGHT     = 0b00000010;
        const TOP       = 0b00000100;
        const BOTTOM       = 0b00001000;
    }
}

pub trait RectUtils
{
    fn inset(&self, amount: usize, edges: Edges) -> Rect;
}

impl RectUtils for Rect
{
    fn inset(&self, amount: usize, edges: Edges) -> Rect
    {
        let mut x: i32 = self.x.into();
        let mut y: i32 = self.y.into();
        let mut w: i32 = self.width.into();
        let mut h: i32 = self.height.into();

        let amount: i32 = amount as i32;

        if edges.intersects(Edges::LEFT)
        {
            x += amount;
            w -= amount;
        }

        if edges.intersects(Edges::RIGHT)
        {
            w -= amount;
        }

        if edges.intersects(Edges::TOP)
        {
            y += amount;
            h -= amount;
        }

        if edges.intersects(Edges::BOTTOM)
        {
            h -= amount;
        }

        w = std::cmp::max(w, 0);
        h = std::cmp::max(h, 0);

        Rect {
            x: x as u16,
            y: y as u16,
            width: w as u16,
            height: h as u16,
        }
    }
}
