use ones::info_module;

#[inline]
#[allow(unused)]
fn info<M>(msg: impl IntoIterator<Item = M>) 
    where M: AsRef<str>,
{
    info_module("address space", msg);
}

mod config {
    /// 单位：页
    pub const _STACK_SIZE: usize = 2;
}