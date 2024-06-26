#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `WDRSTT` writer - Watchdog Restart"]
pub type WdrsttW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Password."]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "165: Writing any other value in this field aborts the write operation."]
    Passwd = 165,
}
impl From<Key> for u8 {
    #[inline(always)]
    fn from(variant: Key) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key {
    type Ux = u8;
}
impl crate::IsEnum for Key {}
#[doc = "Field `KEY` writer - Password."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Passwd)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Restart"]
    #[inline(always)]
    #[must_use]
    pub fn wdrstt(&mut self) -> WdrsttW<CrSpec> {
        WdrsttW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Password."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<CrSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Control Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
