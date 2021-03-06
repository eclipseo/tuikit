use super::split::Split;
use super::Size;
use crate::attr::Attr;
use crate::canvas::{BoundedCanvas, Canvas, Result};
use crate::cell::Cell;
use crate::draw::Draw;

///! A Win is like a div in HTML, it has its margin/padding, and border
pub struct Win<'a> {
    margin_top: Size,
    margin_right: Size,
    margin_bottom: Size,
    margin_left: Size,

    padding_top: Size,
    padding_right: Size,
    padding_bottom: Size,
    padding_left: Size,

    border_top: bool,
    border_right: bool,
    border_bottom: bool,
    border_left: bool,

    border_top_attr: Attr,
    border_right_attr: Attr,
    border_bottom_attr: Attr,
    border_left_attr: Attr,

    basis: Size,
    grow: usize,
    shrink: usize,

    inner: &'a Draw,
}

// Builder
impl<'a> Win<'a> {
    pub fn new(draw: &'a Draw) -> Self {
        Self {
            margin_top: Default::default(),
            margin_right: Default::default(),
            margin_bottom: Default::default(),
            margin_left: Default::default(),
            padding_top: Default::default(),
            padding_right: Default::default(),
            padding_bottom: Default::default(),
            padding_left: Default::default(),
            border_top: false,
            border_right: false,
            border_bottom: false,
            border_left: false,
            border_top_attr: Default::default(),
            border_right_attr: Default::default(),
            border_bottom_attr: Default::default(),
            border_left_attr: Default::default(),
            basis: Size::Default,
            grow: 1,
            shrink: 1,
            inner: draw,
        }
    }

    pub fn margin_top(mut self, margin_top: impl Into<Size>) -> Self {
        self.margin_top = margin_top.into();
        self
    }

    pub fn margin_right(mut self, margin_right: impl Into<Size>) -> Self {
        self.margin_right = margin_right.into();
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: impl Into<Size>) -> Self {
        self.margin_bottom = margin_bottom.into();
        self
    }

    pub fn margin_left(mut self, margin_left: impl Into<Size>) -> Self {
        self.margin_left = margin_left.into();
        self
    }

    pub fn margin(mut self, margin: impl Into<Size>) -> Self {
        let margin = margin.into();
        self.margin_top = margin;
        self.margin_right = margin;
        self.margin_bottom = margin;
        self.margin_left = margin;
        self
    }

    pub fn padding_top(mut self, padding_top: impl Into<Size>) -> Self {
        self.padding_top = padding_top.into();
        self
    }

    pub fn padding_right(mut self, padding_right: impl Into<Size>) -> Self {
        self.padding_right = padding_right.into();
        self
    }

    pub fn padding_bottom(mut self, padding_bottom: impl Into<Size>) -> Self {
        self.padding_bottom = padding_bottom.into();
        self
    }

    pub fn padding_left(mut self, padding_left: impl Into<Size>) -> Self {
        self.padding_left = padding_left.into();
        self
    }

    pub fn padding(mut self, padding: impl Into<Size>) -> Self {
        let padding = padding.into();
        self.padding_top = padding;
        self.padding_right = padding;
        self.padding_bottom = padding;
        self.padding_left = padding;
        self
    }

    pub fn border_top(mut self, border_top: bool) -> Self {
        self.border_top = border_top;
        self
    }

    pub fn border_right(mut self, border_right: bool) -> Self {
        self.border_right = border_right;
        self
    }

    pub fn border_bottom(mut self, border_bottom: bool) -> Self {
        self.border_bottom = border_bottom;
        self
    }

    pub fn border_left(mut self, border_left: bool) -> Self {
        self.border_left = border_left;
        self
    }

    pub fn border(mut self, border: bool) -> Self {
        self.border_top = border;
        self.border_right = border;
        self.border_bottom = border;
        self.border_left = border;
        self
    }

    pub fn border_top_attr(mut self, border_top_attr: impl Into<Attr>) -> Self {
        self.border_top_attr = border_top_attr.into();
        self
    }

    pub fn border_right_attr(mut self, border_right_attr: impl Into<Attr>) -> Self {
        self.border_right_attr = border_right_attr.into();
        self
    }

    pub fn border_bottom_attr(mut self, border_bottom_attr: impl Into<Attr>) -> Self {
        self.border_bottom_attr = border_bottom_attr.into();
        self
    }

    pub fn border_left_attr(mut self, border_left_attr: impl Into<Attr>) -> Self {
        self.border_left_attr = border_left_attr.into();
        self
    }

    pub fn border_attr(mut self, attr: impl Into<Attr>) -> Self {
        let attr = attr.into();
        self.border_top_attr = attr;
        self.border_right_attr = attr;
        self.border_bottom_attr = attr;
        self.border_left_attr = attr;
        self
    }

    pub fn basis(mut self, basis: impl Into<Size>) -> Self {
        self.basis = basis.into();
        self
    }

    pub fn grow(mut self, grow: usize) -> Self {
        self.grow = grow;
        self
    }

    pub fn shrink(mut self, shrink: usize) -> Self {
        self.shrink = shrink;
        self
    }
}

impl<'a> Win<'a> {
    /// draw border and return the position & size of the inner canvas
    /// (top, left, width, height)
    fn draw_border(
        &self,
        top: usize,
        left: usize,
        width: usize,
        height: usize,
        canvas: &mut Canvas,
    ) -> Result<(usize, usize, usize, usize)> {
        if self.border_top || self.border_bottom {
            if (height < 1) || (self.border_top && self.border_bottom && height < 2) {
                return Err("not enough height for border".into());
            }
        }

        if self.border_left || self.border_right {
            if (width < 1) || (self.border_left && self.border_right && width < 2) {
                return Err("not enough width for border".into());
            }
        }

        let bottom = top + height - 1;
        let right = left + width - 1;

        // draw border top
        if self.border_top {
            let _ = canvas.print_with_attr(top, left, &"─".repeat(width), self.border_top_attr);
        }

        if self.border_bottom {
            let _ =
                canvas.print_with_attr(bottom, left, &"─".repeat(width), self.border_bottom_attr);
        }

        if self.border_left {
            for i in top..(top + height) {
                let _ = canvas.print_with_attr(i, left, "│", self.border_left_attr);
            }
        }

        if self.border_right {
            for i in top..(top + height) {
                let _ = canvas.print_with_attr(i, right, "│", self.border_right_attr);
            }
        }

        // draw 4 corners if necessary

        if self.border_top && self.border_left {
            let _ = canvas.put_cell(
                top,
                left,
                Cell::default().ch('┌').attribute(self.border_top_attr),
            );
        }

        if self.border_top && self.border_right {
            let _ = canvas.put_cell(
                top,
                right,
                Cell::default().ch('┐').attribute(self.border_top_attr),
            );
        }

        if self.border_bottom && self.border_left {
            let _ = canvas.put_cell(
                bottom,
                left,
                Cell::default().ch('└').attribute(self.border_bottom_attr),
            );
        }

        if self.border_bottom && self.border_right {
            let _ = canvas.put_cell(
                bottom,
                right,
                Cell::default().ch('┘').attribute(self.border_bottom_attr),
            );
        }

        // re-calculate the position & size
        let top = if self.border_top { top + 1 } else { top };
        let left = if self.border_left { left + 1 } else { left };
        let width = if self.border_left { width - 1 } else { width };
        let width = if self.border_right { width - 1 } else { width };
        let height = if self.border_top { height - 1 } else { height };
        let height = if self.border_bottom {
            height - 1
        } else {
            height
        };

        Ok((top, left, width, height))
    }
}

impl<'a> Draw for Win<'a> {
    /// Reserve margin & padding, draw border.
    fn draw(&self, canvas: &mut Canvas) -> Result<()> {
        let (width, height) = canvas.size()?;

        let margin_top = self.margin_top.calc_fixed_size(height, 0);
        let margin_right = self.margin_right.calc_fixed_size(width, 0);
        let margin_bottom = self.margin_bottom.calc_fixed_size(height, 0);
        let margin_left = self.margin_left.calc_fixed_size(width, 0);

        let padding_top = self.padding_top.calc_fixed_size(height, 0);
        let padding_right = self.padding_right.calc_fixed_size(width, 0);
        let padding_bottom = self.padding_bottom.calc_fixed_size(height, 0);
        let padding_left = self.padding_left.calc_fixed_size(width, 0);

        if margin_top + margin_bottom >= height || margin_left + margin_right >= width {
            return Err("margin takes too much screen, won't draw".into());
        }

        // reserve margin

        let top = margin_top;
        let left = margin_left;
        let width = width - (margin_left + margin_right);
        let height = height - (margin_top + margin_bottom);

        let (top, left, width, height) = self.draw_border(top, left, width, height, canvas)?;

        // reserve padding
        if padding_top + padding_bottom >= height || padding_left + padding_right >= width {
            return Err("padding takes too much screen, won't draw".into());
        }

        let top = top + padding_top;
        let left = left + padding_left;
        let width = width - (padding_left + padding_right);
        let height = height - (padding_top + padding_bottom);

        let mut new_canvas = BoundedCanvas::new(top, left, width, height, canvas);
        self.inner.draw(&mut new_canvas)
    }

    fn size_hint(&self) -> (Option<usize>, Option<usize>) {
        // plus border size
        let (width, height) = self.inner.size_hint();
        let width = width.map(|mut w| {
            w += if self.border_left { 1 } else { 0 };
            w += if self.border_right { 1 } else { 0 };
            w
        });

        let height = height.map(|mut h| {
            h += if self.border_top { 1 } else { 0 };
            h += if self.border_bottom { 1 } else { 0 };
            h
        });

        (width, height)
    }
}

impl<'a> Split for Win<'a> {
    fn get_basis(&self) -> Size {
        self.basis
    }

    fn get_grow(&self) -> usize {
        self.grow
    }

    fn get_shrink(&self) -> usize {
        self.shrink
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod test {
    use super::*;

    struct WinHint {
        pub width_hint: Option<usize>,
        pub height_hint: Option<usize>,
    }

    impl Draw for WinHint {
        fn draw(&self, _canvas: &mut Canvas) -> Result<()> {
            unimplemented!()
        }

        fn size_hint(&self) -> (Option<usize>, Option<usize>) {
            (self.width_hint, self.height_hint)
        }
    }

    #[test]
    fn size_hint_for_window_should_include_border() {
        let inner = WinHint {
            width_hint: None,
            height_hint: None,
        };
        let win_border_top = Win::new(&inner).border_top(true);
        assert_eq!((None, None), win_border_top.size_hint());
        let win_border_right = Win::new(&inner).border_right(true);
        assert_eq!((None, None), win_border_right.size_hint());
        let win_border_bottom = Win::new(&inner).border_bottom(true);
        assert_eq!((None, None), win_border_bottom.size_hint());
        let win_border_left = Win::new(&inner).border_left(true);
        assert_eq!((None, None), win_border_left.size_hint());

        let inner = WinHint {
            width_hint: Some(1),
            height_hint: None,
        };
        let win_border_top = Win::new(&inner).border_top(true);
        assert_eq!((Some(1), None), win_border_top.size_hint());
        let win_border_right = Win::new(&inner).border_right(true);
        assert_eq!((Some(2), None), win_border_right.size_hint());
        let win_border_bottom = Win::new(&inner).border_bottom(true);
        assert_eq!((Some(1), None), win_border_bottom.size_hint());
        let win_border_left = Win::new(&inner).border_left(true);
        assert_eq!((Some(2), None), win_border_left.size_hint());

        let inner = WinHint {
            width_hint: None,
            height_hint: Some(1),
        };
        let win_border_top = Win::new(&inner).border_top(true);
        assert_eq!((None, Some(2)), win_border_top.size_hint());
        let win_border_right = Win::new(&inner).border_right(true);
        assert_eq!((None, Some(1)), win_border_right.size_hint());
        let win_border_bottom = Win::new(&inner).border_bottom(true);
        assert_eq!((None, Some(2)), win_border_bottom.size_hint());
        let win_border_left = Win::new(&inner).border_left(true);
        assert_eq!((None, Some(1)), win_border_left.size_hint());
    }
}
