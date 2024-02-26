#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `PROCRST` writer - Processor Reset"]
pub type ProcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERRST` writer - Peripheral Reset"]
pub type PerrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTRST` writer - External Reset"]
pub type ExtrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "System Reset Key"]
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
#[doc = "Field `KEY` writer - System Reset Key"]
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
    #[doc = "Bit 0 - Processor Reset"]
    #[inline(always)]
    #[must_use]
    pub fn procrst(&mut self) -> ProcrstW<CrSpec> {
        ProcrstW::new(self, 0)
    }
    #[doc = "Bit 2 - Peripheral Reset"]
    #[inline(always)]
    #[must_use]
    pub fn perrst(&mut self) -> PerrstW<CrSpec> {
        PerrstW::new(self, 2)
    }
    #[doc = "Bit 3 - External Reset"]
    #[inline(always)]
    #[must_use]
    pub fn extrst(&mut self) -> ExtrstW<CrSpec> {
        ExtrstW::new(self, 3)
    }
    #[doc = "Bits 24:31 - System Reset Key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<CrSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
