use crate::Error;

/// Nonzero pixel dimensions, each at most `i32::MAX`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    width: u32,
    height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Result<Self, Error> {
        if width == 0 || height == 0 || width > i32::MAX as u32 || height > i32::MAX as u32 {
            return Err(Error::InvalidGeometry);
        }
        Ok(Self { width, height })
    }

    pub const fn width(self) -> u32 {
        self.width
    }

    pub const fn height(self) -> u32 {
        self.height
    }
}

/// Half-open rectangle. Negative screen origins are supported. Right/bottom
/// endpoints must remain representable as `i32`; arithmetic uses wider integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    x: i32,
    y: i32,
    size: Size,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Result<Self, Error> {
        let size = Size::new(width, height)?;
        Self::from_size(x, y, size)
    }

    pub fn from_size(x: i32, y: i32, size: Size) -> Result<Self, Error> {
        i32::try_from(i64::from(x) + i64::from(size.width)).map_err(|_| Error::InvalidGeometry)?;
        i32::try_from(i64::from(y) + i64::from(size.height)).map_err(|_| Error::InvalidGeometry)?;
        Ok(Self { x, y, size })
    }

    pub const fn x(self) -> i32 {
        self.x
    }

    pub const fn y(self) -> i32 {
        self.y
    }

    pub const fn size(self) -> Size {
        self.size
    }

    pub const fn width(self) -> u32 {
        self.size.width
    }

    pub const fn height(self) -> u32 {
        self.size.height
    }

    pub fn right(self) -> i64 {
        i64::from(self.x) + i64::from(self.width())
    }

    pub fn bottom(self) -> i64 {
        i64::from(self.y) + i64::from(self.height())
    }

    pub fn contains(self, other: Self) -> bool {
        other.x >= self.x
            && other.y >= self.y
            && other.right() <= self.right()
            && other.bottom() <= self.bottom()
    }

    pub(crate) fn inset(self, gap: u32) -> Result<Self, Error> {
        let double = gap.checked_mul(2).ok_or(Error::InvalidConfig)?;
        let width = self
            .width()
            .checked_sub(double)
            .ok_or(Error::InvalidConfig)?;
        let height = self
            .height()
            .checked_sub(double)
            .ok_or(Error::InvalidConfig)?;
        Self::new(
            coordinate(i64::from(self.x) + i64::from(gap))?,
            coordinate(i64::from(self.y) + i64::from(gap))?,
            width,
            height,
        )
    }

    /// Clamp requested size and position inside usable bounds, retaining minimums.
    pub(crate) fn fit(self, bounds: Self, minimum: Size) -> Result<Self, Error> {
        if minimum.width() > bounds.width() || minimum.height() > bounds.height() {
            return Err(Error::InsufficientSpace);
        }
        let size = Size::new(
            self.width().clamp(minimum.width(), bounds.width()),
            self.height().clamp(minimum.height(), bounds.height()),
        )?;
        Self::from_size(
            coordinate(i64::from(self.x).clamp(
                i64::from(bounds.x),
                bounds.right() - i64::from(size.width()),
            ))?,
            coordinate(i64::from(self.y).clamp(
                i64::from(bounds.y),
                bounds.bottom() - i64::from(size.height()),
            ))?,
            size,
        )
    }
}

pub(crate) fn coordinate(value: i64) -> Result<i32, Error> {
    i32::try_from(value).map_err(|_| Error::InvalidGeometry)
}

/// Exact integer partitions; earlier cells receive remainder pixels. No overlap
/// or lost pixels. Used for tiles and snap regions alike.
pub(crate) fn partition(
    bounds: Rect,
    count: usize,
    gap: u32,
    columns: bool,
    minimum: Size,
) -> Result<Vec<Rect>, Error> {
    if count > crate::MAX_WINDOWS {
        return Err(Error::Capacity);
    }
    if count == 0 {
        return Ok(Vec::new());
    }
    let n = u32::try_from(count).map_err(|_| Error::Capacity)?;
    let axis = if columns {
        bounds.width()
    } else {
        bounds.height()
    };
    let gaps = gap.checked_mul(n - 1).ok_or(Error::InsufficientSpace)?;
    let available = axis.checked_sub(gaps).ok_or(Error::InsufficientSpace)?;
    let base = available / n;
    let required = if columns {
        minimum.width()
    } else {
        minimum.height()
    };
    if base < required || bounds.width() < minimum.width() || bounds.height() < minimum.height() {
        return Err(Error::InsufficientSpace);
    }
    let mut result = Vec::with_capacity(count);
    let mut offset = 0_i64;
    for index in 0..n {
        let length = base + u32::from(index < available % n);
        result.push(if columns {
            Rect::new(
                coordinate(i64::from(bounds.x) + offset)?,
                bounds.y,
                length,
                bounds.height(),
            )?
        } else {
            Rect::new(
                bounds.x,
                coordinate(i64::from(bounds.y) + offset)?,
                bounds.width(),
                length,
            )?
        });
        offset += i64::from(length) + i64::from(gap);
    }
    Ok(result)
}
