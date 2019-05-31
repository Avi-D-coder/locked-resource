use std::io::{Stdin, StdinLock};
use std::mem::transmute;
use std::ops::{Deref, DerefMut};

pub trait WithLock<'l> {
    type Lock;
    fn lock_resource(&'l mut self) -> Self::Lock;
    fn with_lock(self) -> LockedResource<Self, Self::Lock>;
}

pub struct LockedResource<R: ?Sized, L> {
    lock: L,
    resource: R,
}

impl<R: ?Sized, L> Deref for LockedResource<R, L> {
    type Target = L;
    fn deref(&self) -> &L {
        &self.lock
    }
}

impl<R: ?Sized, L> DerefMut for LockedResource<R, L> {
    fn deref_mut(&mut self) -> &mut L {
        &mut self.lock
    }
}

impl<'l, R, L> LockedResource<R, L> {
    pub fn unlock(self) -> R {
        drop(self.lock);
        self.resource
    }
}

impl<'l> WithLock<'l> for Stdin {
    type Lock = StdinLock<'l>;

    fn lock_resource(&'l mut self) -> StdinLock<'l> {
        self.lock()
    }

    fn with_lock(self) -> LockedResource<Stdin, StdinLock<'l>> {
        let lock = unsafe { transmute(self.lock()) };
        LockedResource {
            resource: self,
            lock,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{LockedResource, WithLock};
    use std::io::{stdin, BufRead, Stdin, StdinLock};

    fn use_stdin<'l>(
        mut locked_stdin: LockedResource<Stdin, StdinLock<'l>>,
    ) -> LockedResource<Stdin, StdinLock<'l>> {
        let mut line = String::new();
        locked_stdin.read_line(&mut line).unwrap();
        locked_stdin
    }

    #[allow(dead_code)]
    fn it_type_checks() {
        let mut locked_stdin = stdin().with_lock();
        let mut line = String::new();
        locked_stdin.read_line(&mut line).unwrap();
        use_stdin(locked_stdin);
    }

    #[test]
    fn lock_unlock() {
        {
            stdin().with_lock();
        }
        stdin().with_lock().unlock().lock();
    }
}
