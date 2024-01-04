#[doc = "Register `MR` reader"]
pub type R = crate::R<MR_SPEC>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MR_SPEC>;
#[doc = "Field `PAR` reader - Parity Type"]
pub type PAR_R = crate::FieldReader<PAR_A>;
#[doc = "Parity Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PAR_A {
    #[doc = "0: Even parity"]
    Even = 0,
    #[doc = "1: Odd parity"]
    Odd = 1,
    #[doc = "2: Space: parity forced to 0"]
    Space = 2,
    #[doc = "3: Mark: parity forced to 1"]
    Mark = 3,
    #[doc = "4: No parity"]
    No = 4,
}
impl From<PAR_A> for u8 {
    #[inline(always)]
    fn from(variant: PAR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PAR_A {
    type Ux = u8;
}
impl PAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PAR_A> {
        match self.bits {
            0 => Some(PAR_A::Even),
            1 => Some(PAR_A::Odd),
            2 => Some(PAR_A::Space),
            3 => Some(PAR_A::Mark),
            4 => Some(PAR_A::No),
            _ => None,
        }
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PAR_A::Even
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PAR_A::Odd
    }
    #[doc = "Space: parity forced to 0"]
    #[inline(always)]
    pub fn is_space(&self) -> bool {
        *self == PAR_A::Space
    }
    #[doc = "Mark: parity forced to 1"]
    #[inline(always)]
    pub fn is_mark(&self) -> bool {
        *self == PAR_A::Mark
    }
    #[doc = "No parity"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == PAR_A::No
    }
}
#[doc = "Field `PAR` writer - Parity Type"]
pub type PAR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PAR_A>;
impl<'a, REG> PAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(PAR_A::Even)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(PAR_A::Odd)
    }
    #[doc = "Space: parity forced to 0"]
    #[inline(always)]
    pub fn space(self) -> &'a mut crate::W<REG> {
        self.variant(PAR_A::Space)
    }
    #[doc = "Mark: parity forced to 1"]
    #[inline(always)]
    pub fn mark(self) -> &'a mut crate::W<REG> {
        self.variant(PAR_A::Mark)
    }
    #[doc = "No parity"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(PAR_A::No)
    }
}
#[doc = "Field `CHMODE` reader - Channel Mode"]
pub type CHMODE_R = crate::FieldReader<CHMODE_A>;
#[doc = "Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHMODE_A {
    #[doc = "0: Normal Mode"]
    Normal = 0,
    #[doc = "1: Automatic Echo"]
    Automatic = 1,
    #[doc = "2: Local Loopback"]
    LocalLoopback = 2,
    #[doc = "3: Remote Loopback"]
    RemoteLoopback = 3,
}
impl From<CHMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CHMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHMODE_A {
    type Ux = u8;
}
impl CHMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHMODE_A {
        match self.bits {
            0 => CHMODE_A::Normal,
            1 => CHMODE_A::Automatic,
            2 => CHMODE_A::LocalLoopback,
            3 => CHMODE_A::RemoteLoopback,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CHMODE_A::Normal
    }
    #[doc = "Automatic Echo"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == CHMODE_A::Automatic
    }
    #[doc = "Local Loopback"]
    #[inline(always)]
    pub fn is_local_loopback(&self) -> bool {
        *self == CHMODE_A::LocalLoopback
    }
    #[doc = "Remote Loopback"]
    #[inline(always)]
    pub fn is_remote_loopback(&self) -> bool {
        *self == CHMODE_A::RemoteLoopback
    }
}
#[doc = "Field `CHMODE` writer - Channel Mode"]
pub type CHMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CHMODE_A>;
impl<'a, REG> CHMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(CHMODE_A::Normal)
    }
    #[doc = "Automatic Echo"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(CHMODE_A::Automatic)
    }
    #[doc = "Local Loopback"]
    #[inline(always)]
    pub fn local_loopback(self) -> &'a mut crate::W<REG> {
        self.variant(CHMODE_A::LocalLoopback)
    }
    #[doc = "Remote Loopback"]
    #[inline(always)]
    pub fn remote_loopback(self) -> &'a mut crate::W<REG> {
        self.variant(CHMODE_A::RemoteLoopback)
    }
}
impl R {
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    pub fn par(&self) -> PAR_R {
        PAR_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    pub fn chmode(&self) -> CHMODE_R {
        CHMODE_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    #[must_use]
    pub fn par(&mut self) -> PAR_W<MR_SPEC> {
        PAR_W::new(self, 9)
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    #[must_use]
    pub fn chmode(&mut self) -> CHMODE_W<MR_SPEC> {
        CHMODE_W::new(self, 14)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: u32 = 0;
}
