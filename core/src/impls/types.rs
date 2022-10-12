use crate::types::*;

impl core::fmt::Display for Rect {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.left.fmt(f)?;
        ','.fmt(f)?;
        self.top.fmt(f)?;
        ' '.fmt(f)?;
        self.width.fmt(f)?;
        'x'.fmt(f)?;
        self.height.fmt(f)
    }
}

impl core::fmt::Display for Fract {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.numerator.fmt(f)?;
        '/'.fmt(f)?;
        self.denominator.fmt(f)
    }
}

impl core::fmt::Display for Area {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.width.fmt(f)?;
        'x'.fmt(f)?;
        self.height.fmt(f)
    }
}

impl core::fmt::Display for TimeCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.type_.fmt(f)?;
        if !self.flags.is_none() {
            ' '.fmt(f)?;
            self.flags.fmt(f)?;
        }
        ' '.fmt(f)?;
        write!(
            f,
            "{:02}:{:02}:{:02},{:03}",
            self.hours, self.minutes, self.seconds, self.frames
        )
    }
}
