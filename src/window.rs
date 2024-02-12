pub trait Surface<'a> {
    fn get_data_mut<'b>(&'b mut self) -> &'b mut [u32];
    fn get_data<'b>(&'b self) -> &'b [u32];
    fn get_extent(&self) -> crate::math::Vec2<usize>;
}