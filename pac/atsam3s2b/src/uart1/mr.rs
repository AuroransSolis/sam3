#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Parity Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Par {
    #[doc = "0: Even Parity"]
    Even = 0,
    #[doc = "1: Odd Parity"]
    Odd = 1,
    #[doc = "2: Space: parity forced to 0"]
    Space = 2,
    #[doc = "3: Mark: parity forced to 1"]
    Mark = 3,
    #[doc = "4: No Parity"]
    No = 4,
}
impl From<Par> for u8 {
    #[inline(always)]
    fn from(variant: Par) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Par {
    type Ux = u8;
}
impl crate::IsEnum for Par {}
#[doc = "Field `PAR` reader - Parity Type"]
pub type ParR = crate::FieldReader<Par>;
impl ParR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Par> {
        match self.bits {
            0 => Some(Par::Even),
            1 => Some(Par::Odd),
            2 => Some(Par::Space),
            3 => Some(Par::Mark),
            4 => Some(Par::No),
            _ => None,
        }
    }
    #[doc = "Even Parity"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == Par::Even
    }
    #[doc = "Odd Parity"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Par::Odd
    }
    #[doc = "Space: parity forced to 0"]
    #[inline(always)]
    pub fn is_space(&self) -> bool {
        *self == Par::Space
    }
    #[doc = "Mark: parity forced to 1"]
    #[inline(always)]
    pub fn is_mark(&self) -> bool {
        *self == Par::Mark
    }
    #[doc = "No Parity"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Par::No
    }
}
#[doc = "Field `PAR` writer - Parity Type"]
pub type ParW<'a, REG> = crate::FieldWriter<'a, REG, 3, Par>;
impl<'a, REG> ParW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Even Parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(Par::Even)
    }
    #[doc = "Odd Parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(Par::Odd)
    }
    #[doc = "Space: parity forced to 0"]
    #[inline(always)]
    pub fn space(self) -> &'a mut crate::W<REG> {
        self.variant(Par::Space)
    }
    #[doc = "Mark: parity forced to 1"]
    #[inline(always)]
    pub fn mark(self) -> &'a mut crate::W<REG> {
        self.variant(Par::Mark)
    }
    #[doc = "No Parity"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Par::No)
    }
}
#[doc = "Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chmode {
    #[doc = "0: Normal Mode"]
    Normal = 0,
    #[doc = "1: Automatic Echo"]
    Automatic = 1,
    #[doc = "2: Local Loopback"]
    LocalLoopback = 2,
    #[doc = "3: Remote Loopback"]
    RemoteLoopback = 3,
}
impl From<Chmode> for u8 {
    #[inline(always)]
    fn from(variant: Chmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chmode {
    type Ux = u8;
}
impl crate::IsEnum for Chmode {}
#[doc = "Field `CHMODE` reader - Channel Mode"]
pub type ChmodeR = crate::FieldReader<Chmode>;
impl ChmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chmode {
        match self.bits {
            0 => Chmode::Normal,
            1 => Chmode::Automatic,
            2 => Chmode::LocalLoopback,
            3 => Chmode::RemoteLoopback,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Chmode::Normal
    }
    #[doc = "Automatic Echo"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == Chmode::Automatic
    }
    #[doc = "Local Loopback"]
    #[inline(always)]
    pub fn is_local_loopback(&self) -> bool {
        *self == Chmode::LocalLoopback
    }
    #[doc = "Remote Loopback"]
    #[inline(always)]
    pub fn is_remote_loopback(&self) -> bool {
        *self == Chmode::RemoteLoopback
    }
}
#[doc = "Field `CHMODE` writer - Channel Mode"]
pub type ChmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Chmode, crate::Safe>;
impl<'a, REG> ChmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Chmode::Normal)
    }
    #[doc = "Automatic Echo"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(Chmode::Automatic)
    }
    #[doc = "Local Loopback"]
    #[inline(always)]
    pub fn local_loopback(self) -> &'a mut crate::W<REG> {
        self.variant(Chmode::LocalLoopback)
    }
    #[doc = "Remote Loopback"]
    #[inline(always)]
    pub fn remote_loopback(self) -> &'a mut crate::W<REG> {
        self.variant(Chmode::RemoteLoopback)
    }
}
impl R {
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    pub fn par(&self) -> ParR {
        ParR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    pub fn chmode(&self) -> ChmodeR {
        ChmodeR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    #[must_use]
    pub fn par(&mut self) -> ParW<MrSpec> {
        ParW::new(self, 9)
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    #[must_use]
    pub fn chmode(&mut self) -> ChmodeW<MrSpec> {
        ChmodeW::new(self, 14)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MrSpec {
    const RESET_VALUE: u32 = 0;
}
