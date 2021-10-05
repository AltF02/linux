// SPDX-License-Identifier: GPL-2.0

// TODO: Document.
//! Generic devices.

use crate::{
    bindings,
    revocable::{Revocable, RevocableGuard},
};
use core::ops::{Deref, DerefMut};

pub trait Device {
    fn raw(&self) -> *const bindings::device;
}

pub trait DataRemoval {
    fn device_remove(&self);
}

pub struct Data<Reg, Res, Gen> {
    revocable: Revocable<(Reg, Res)>,
    general: Gen,
}

impl<Reg, Res, Gen> Data<Reg, Res, Gen> {
    pub fn new(regs: Reg, res: Res, gen: Gen) -> Self {
        Self {
            revocable: Revocable::new((regs, res)),
            general: gen,
        }
    }

    pub fn resources(&self) -> Option<RevocableGuard<'_, Res>> {
        Some(self.revocable.try_access()?.map(|r| &r.1))
    }

    pub fn registrations(&self) -> Option<RevocableGuard<'_, Reg>> {
        Some(self.revocable.try_access()?.map(|r| &r.0))
    }
}

impl<Reg, Res, Gen> DataRemoval for Data<Reg, Res, Gen> {
    fn device_remove(&self) {
        // The drop order guarantees that registrations will be dropped first, which makes
        // resources available while the unregistration is ongoing. Resources are then dropped.
        //
        // The general data is still available to all references to the device data, but they
        // cannot access resources nor registration anymore.
        self.revocable.revoke();
    }
}

impl<Reg, Res, Gen> Deref for Data<Reg, Res, Gen> {
    type Target = Gen;

    fn deref(&self) -> &Gen {
        &self.general
    }
}

impl<Reg, Res, Gen> DerefMut for Data<Reg, Res, Gen> {
    fn deref_mut(&mut self) -> &mut Gen {
        &mut self.general
    }
}
