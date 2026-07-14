pub(crate) trait Handler {
    type Arguments;
    type Return;

    fn handle(&self, val: Self::Arguments) -> Self::Return;
}
