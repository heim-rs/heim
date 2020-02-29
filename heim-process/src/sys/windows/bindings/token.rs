use heim_common::sys::windows::Handle;
use heim_common::Result;
use heim_host::User;
//use heim_host::os::windows::UserExt;

pub struct Token(Handle);

impl Token {
    pub fn open(process: &Handle) -> Result<Self> {
        unimplemented!();
    }

    pub fn user(&self) -> Result<User> {
        // let sid = self.get_sid_somehow();
        //
        // Really should call it `try_from_sid`:
        // UserExt::try_from_sid(sid)
        unimplemented!()
    }
}
