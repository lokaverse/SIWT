use crate::state::State;
use crate::types::*;

thread_local! {
    static MEMORY_MANAGER: MemoryManagerType = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
    static STATE: State = State::default();
    static MESSAGES: RefCell<Messages> = RefCell::new(Messages::new());
    static SETTING: SettingStateType = Setting::init(allocate(0));
    static ACCOUNTS: AccountsStateType = Accounts::init([
        allocate(1),
        allocate(2),
        allocate(3),
    ]);
    static GLOBALS: RefCell<Globals> = Globals::init(allocate(4));
}

fn allocate(id: u8) -> VM {
    MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(id)))
}

pub(crate) fn state<F: FnOnce(&State) -> R, R>(f: F) -> R {
    STATE.with(f)
}

pub(crate) mod setting {
    use super::*;

    pub fn get() -> Setting {
        borrow(Clone::clone)
    }

    pub fn set(setting: Setting) {
        SETTING.with_borrow_mut(|cell| cell.set(setting).unwrap());
    }

    pub fn borrow<F: FnOnce(&Setting) -> R, R>(f: F) -> R {
        SETTING.with_borrow(|cell| f(cell.get()))
    }
}

pub(crate) mod accounts {
    use super::*;

    pub fn borrow<F: FnOnce(&Accounts) -> R, R>(f: F) -> R {
        ACCOUNTS.with_borrow(f)
    }

    pub fn borrow_mut<F: FnOnce(&mut Accounts) -> R, R>(f: F) -> R {
        ACCOUNTS.with_borrow_mut(f)
    }
}

pub(crate) mod messages {
    use super::*;

    pub fn borrow<F: FnOnce(&Messages) -> R, R>(f: F) -> R {
        MESSAGES.with_borrow(f)
    }

    pub fn borrow_mut<F: FnOnce(&mut Messages) -> R, R>(f: F) -> R {
        MESSAGES.with_borrow_mut(f)
    }
}

pub(crate) mod globals {
    use super::*;

    pub fn borrow<F: FnOnce(&Globals) -> R, R>(f: F) -> R {
        GLOBALS.with_borrow(f)
    }

    pub fn borrow_mut<F: FnOnce(&mut Globals) -> R, R>(f: F) -> R {
        GLOBALS.with_borrow_mut(f)
    }
}
