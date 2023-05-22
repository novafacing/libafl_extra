//! MapHashingObserver prints the hash of the map it is observing before and after each run.

use crc32fast::hash;
use libafl::{
    prelude::{
        AsIter, AsIterMut, AsMutSlice, AsSlice, DifferentialObserver, HasLen, MapObserver, Named,
        Observer, ObserversTuple, Truncate, UsesInput,
    },
    Error, ErrorBacktrace,
};
use log::info;
use serde::{Deserialize, Serialize};

use super::count_class::init_count_class_16;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MapHashingObserverSettings {
    assert_different: bool,
    err_if_same: bool,
    print_crcs: bool,
    log_crcs: bool,
}

impl MapHashingObserverSettings {
    pub fn new(
        assert_different: bool,
        err_if_same: bool,
        print_crcs: bool,
        log_crcs: bool,
    ) -> Self {
        Self {
            assert_different,
            err_if_same,
            print_crcs,
            log_crcs,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(bound = "M: serde::de::DeserializeOwned")]
pub struct MapHashingObserver<M>
where
    M: Serialize,
{
    base: M,
    settings: MapHashingObserverSettings,
    last_pre_hash: Option<u32>,
}

impl<S, M> Observer<S> for MapHashingObserver<M>
where
    M: MapObserver<Entry = u8> + Observer<S> + AsMutSlice<Entry = u8>,
    S: UsesInput,
{
    #[inline]
    fn pre_exec(&mut self, state: &mut S, input: &<S as UsesInput>::Input) -> Result<(), Error> {
        self.base.pre_exec(state, input)?;
        let pre_hash = hash(self.base.as_mut_slice());

        if self.settings.print_crcs {
            println!("pre_hash for {}: {:#x}", self.name(), pre_hash);
        }

        if self.settings.log_crcs {
            info!("pre_hash for {}: {:#x}", self.name(), pre_hash);
        }

        self.last_pre_hash = Some(pre_hash);

        Ok(())
    }

    #[inline]
    fn post_exec(
        &mut self,
        _state: &mut S,
        _input: &<S as UsesInput>::Input,
        _exit_kind: &libafl::prelude::ExitKind,
    ) -> Result<(), Error> {
        let post_hash = hash(self.base.as_mut_slice());

        if self.settings.print_crcs {
            println!("post_hash for {}: {:#x}", self.name(), post_hash);
        }

        if self.settings.log_crcs {
            info!("post_hash for {}: {:#x}", self.name(), post_hash);
        }

        if let Some(pre_hash) = self.last_pre_hash {
            if self.settings.err_if_same && post_hash == pre_hash {
                return Err(Error::Unknown(
                    format!(
                        "Pre hash {} and post hash {} for map {} should be different",
                        pre_hash,
                        post_hash,
                        self.name()
                    ),
                    ErrorBacktrace::default(),
                ));
            }

            if self.settings.assert_different {
                assert_ne!(
                    post_hash,
                    pre_hash,
                    "Pre hash {} and post hash {} for map {} should be different",
                    pre_hash,
                    post_hash,
                    self.name()
                );
            }
        }

        Ok(())
    }
}

impl<M> Named for MapHashingObserver<M>
where
    M: Named + Serialize + serde::de::DeserializeOwned,
{
    #[inline]
    fn name(&self) -> &str {
        self.base.name()
    }
}

impl<M> HasLen for MapHashingObserver<M>
where
    M: MapObserver,
{
    #[inline]
    fn len(&self) -> usize {
        self.base.len()
    }
}

impl<M> MapObserver for MapHashingObserver<M>
where
    M: MapObserver<Entry = u8>,
{
    type Entry = u8;

    #[inline]
    fn initial(&self) -> u8 {
        self.base.initial()
    }

    #[inline]
    fn usable_count(&self) -> usize {
        self.base.usable_count()
    }

    #[inline]
    fn get(&self, idx: usize) -> &u8 {
        self.base.get(idx)
    }

    #[inline]
    fn get_mut(&mut self, idx: usize) -> &mut u8 {
        self.base.get_mut(idx)
    }

    /// Count the set bytes in the map
    fn count_bytes(&self) -> u64 {
        self.base.count_bytes()
    }

    /// Reset the map
    #[inline]
    fn reset_map(&mut self) -> Result<(), Error> {
        self.base.reset_map()
    }

    fn hash(&self) -> u64 {
        self.base.hash()
    }
    fn to_vec(&self) -> Vec<u8> {
        self.base.to_vec()
    }

    fn how_many_set(&self, indexes: &[usize]) -> usize {
        self.base.how_many_set(indexes)
    }
}

impl<M> Truncate for MapHashingObserver<M>
where
    M: Named + Serialize + serde::de::DeserializeOwned + Truncate,
{
    fn truncate(&mut self, new_len: usize) {
        self.base.truncate(new_len);
    }
}

impl<M> AsSlice for MapHashingObserver<M>
where
    M: MapObserver + AsSlice,
{
    type Entry = <M as AsSlice>::Entry;
    #[inline]
    fn as_slice(&self) -> &[Self::Entry] {
        self.base.as_slice()
    }
}

impl<M> AsMutSlice for MapHashingObserver<M>
where
    M: MapObserver + AsMutSlice,
{
    type Entry = <M as AsMutSlice>::Entry;
    #[inline]
    fn as_mut_slice(&mut self) -> &mut [Self::Entry] {
        self.base.as_mut_slice()
    }
}

impl<M> MapHashingObserver<M>
where
    M: Serialize + serde::de::DeserializeOwned,
{
    /// Creates a new [`MapObserver`]
    pub fn new(base: M, settings: MapHashingObserverSettings) -> Self {
        init_count_class_16();
        Self {
            base,
            settings,
            last_pre_hash: None,
        }
    }
}

impl<'it, M> AsIter<'it> for MapHashingObserver<M>
where
    M: Named + Serialize + serde::de::DeserializeOwned + AsIter<'it, Item = u8>,
{
    type Item = u8;
    type IntoIter = <M as AsIter<'it>>::IntoIter;

    fn as_iter(&'it self) -> Self::IntoIter {
        self.base.as_iter()
    }
}

impl<'it, M> AsIterMut<'it> for MapHashingObserver<M>
where
    M: Named + Serialize + serde::de::DeserializeOwned + AsIterMut<'it, Item = u8>,
{
    type Item = u8;
    type IntoIter = <M as AsIterMut<'it>>::IntoIter;

    fn as_iter_mut(&'it mut self) -> Self::IntoIter {
        self.base.as_iter_mut()
    }
}

impl<'it, M> IntoIterator for &'it MapHashingObserver<M>
where
    M: Named + Serialize + serde::de::DeserializeOwned,
    &'it M: IntoIterator<Item = &'it u8>,
{
    type Item = &'it u8;
    type IntoIter = <&'it M as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.base.into_iter()
    }
}

impl<'it, M> IntoIterator for &'it mut MapHashingObserver<M>
where
    M: Named + Serialize + serde::de::DeserializeOwned,
    &'it mut M: IntoIterator<Item = &'it mut u8>,
{
    type Item = &'it mut u8;
    type IntoIter = <&'it mut M as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.base.into_iter()
    }
}

impl<M, OTA, OTB, S> DifferentialObserver<OTA, OTB, S> for MapHashingObserver<M>
where
    M: DifferentialObserver<OTA, OTB, S>
        + MapObserver<Entry = u8>
        + Serialize
        + AsMutSlice<Entry = u8>,
    OTA: ObserversTuple<S>,
    OTB: ObserversTuple<S>,
    S: UsesInput,
{
    fn pre_observe_first(&mut self, observers: &mut OTA) -> Result<(), Error> {
        self.base.pre_observe_first(observers)
    }

    fn post_observe_first(&mut self, observers: &mut OTA) -> Result<(), Error> {
        self.base.post_observe_first(observers)
    }

    fn pre_observe_second(&mut self, observers: &mut OTB) -> Result<(), Error> {
        self.base.pre_observe_second(observers)
    }

    fn post_observe_second(&mut self, observers: &mut OTB) -> Result<(), Error> {
        self.base.post_observe_second(observers)
    }
}
