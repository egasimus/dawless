use crate::{*, crossterm::{cursor::*,style::*}};

/// A border around another widget
#[derive(Copy, Clone, Default)]
pub struct Border<T: BorderStyle, W: Widget>(pub T, pub W);

impl<T: BorderStyle, W: Widget> Widget for Border<T, W> {
    impl_render!(self, out, area => {
        let Area(x, y, w, h) = area;
        if w == 0 || h == 0 { return Ok((0, 0)) }
        let bg = Color::AnsiValue(235);
        //Filled(bg).layout(max)?.render(term, Area(Point(x, y), Size(w, h)))?;
        let top_edge    = "▇".repeat((w - 1) as usize);
        let bottom_edge = "▁".repeat((w - 1) as usize);
        let left_edge   = "▊";
        let right_edge  = "▎";
        out.queue(ResetColor)?
            .queue(SetBackgroundColor(Color::AnsiValue(16)))?
            .queue(SetForegroundColor(bg))?
            .queue(MoveTo(x, y))?
            .queue(Print(&top_edge))?;
        for y in y..y+h {
            out.queue(MoveTo(x, y))?.queue(Print(&left_edge))?;
        }
        out.queue(SetBackgroundColor(bg))?
            .queue(SetForegroundColor(Color::AnsiValue(240)))?
            .queue(MoveTo(x+1, y+h-1))?.queue(Print(&bottom_edge))?;
        for y in y..y+h {
            out.queue(MoveTo(x+w-1, y))?.queue(Print(&right_edge))?;
        }
        self.1.render(out, area)
    });
}

/// A border character, and whether it should be rendered inverse.
pub type BorderChar = (char, bool);

/// A set of border characters.
pub trait BorderStyle {
    /** Top left */     fn tl () -> BorderChar;
    /** Top */          fn t  () -> BorderChar;
    /** Top right */    fn tr () -> BorderChar;
    /** Right */        fn r  () -> BorderChar;
    /** Bottom right */ fn br () -> BorderChar;
    /** Bottom */       fn b  () -> BorderChar;
    /** Bottom left */  fn bl () -> BorderChar;
    /** Left */         fn l  () -> BorderChar;
}

/// An inset border with more vertical space.
pub struct InsetTall;

impl BorderStyle for InsetTall {
    fn t  () -> BorderChar { ('▇', true)  }
    fn tr () -> BorderChar { ('▎', false) }
    fn r  () -> BorderChar { ('▎', false) }
    fn br () -> BorderChar { ('▎', false) }
    fn b  () -> BorderChar { ('▁', false) }
    fn bl () -> BorderChar { ('▊', true)  }
    fn l  () -> BorderChar { ('▊', true)  }
    fn tl () -> BorderChar { ('▊', true)  }
}
