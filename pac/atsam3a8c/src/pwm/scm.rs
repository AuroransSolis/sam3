#[doc = "Register `SCM` reader"]
pub type R = crate::R<SCM_SPEC>;
#[doc = "Register `SCM` writer"]
pub type W = crate::W<SCM_SPEC>;
#[doc = "Field `SYNC0` reader - Synchronous Channel 0"]
pub type SYNC0_R = crate::BitReader;
#[doc = "Field `SYNC0` writer - Synchronous Channel 0"]
pub type SYNC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNC1` reader - Synchronous Channel 1"]
pub type SYNC1_R = crate::BitReader;
#[doc = "Field `SYNC1` writer - Synchronous Channel 1"]
pub type SYNC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNC2` reader - Synchronous Channel 2"]
pub type SYNC2_R = crate::BitReader;
#[doc = "Field `SYNC2` writer - Synchronous Channel 2"]
pub type SYNC2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNC3` reader - Synchronous Channel 3"]
pub type SYNC3_R = crate::BitReader;
#[doc = "Field `SYNC3` writer - Synchronous Channel 3"]
pub type SYNC3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNC4` reader - Synchronous Channel 4"]
pub type SYNC4_R = crate::BitReader;
#[doc = "Field `SYNC4` writer - Synchronous Channel 4"]
pub type SYNC4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNC5` reader - Synchronous Channel 5"]
pub type SYNC5_R = crate::BitReader;
#[doc = "Field `SYNC5` writer - Synchronous Channel 5"]
pub type SYNC5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNC6` reader - Synchronous Channel 6"]
pub type SYNC6_R = crate::BitReader;
#[doc = "Field `SYNC6` writer - Synchronous Channel 6"]
pub type SYNC6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNC7` reader - Synchronous Channel 7"]
pub type SYNC7_R = crate::BitReader;
#[doc = "Field `SYNC7` writer - Synchronous Channel 7"]
pub type SYNC7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPDM` reader - Synchronous Channels Update Mode"]
pub type UPDM_R = crate::FieldReader<UPDM_A>;
#[doc = "Synchronous Channels Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UPDM_A {
    #[doc = "0: Manual write of double buffer registers and manual update of synchronous channels"]
    Mode0 = 0,
    #[doc = "1: Manual write of double buffer registers and automatic update of synchronous channels"]
    Mode1 = 1,
    #[doc = "2: Automatic write of duty-cycle update registers by the PDC and automatic update of synchronous channels"]
    Mode2 = 2,
}
impl From<UPDM_A> for u8 {
    #[inline(always)]
    fn from(variant: UPDM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UPDM_A {
    type Ux = u8;
}
impl UPDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UPDM_A> {
        match self.bits {
            0 => Some(UPDM_A::Mode0),
            1 => Some(UPDM_A::Mode1),
            2 => Some(UPDM_A::Mode2),
            _ => None,
        }
    }
    #[doc = "Manual write of double buffer registers and manual update of synchronous channels"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == UPDM_A::Mode0
    }
    #[doc = "Manual write of double buffer registers and automatic update of synchronous channels"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == UPDM_A::Mode1
    }
    #[doc = "Automatic write of duty-cycle update registers by the PDC and automatic update of synchronous channels"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == UPDM_A::Mode2
    }
}
#[doc = "Field `UPDM` writer - Synchronous Channels Update Mode"]
pub type UPDM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, UPDM_A>;
impl<'a, REG, const O: u8> UPDM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Manual write of double buffer registers and manual update of synchronous channels"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(UPDM_A::Mode0)
    }
    #[doc = "Manual write of double buffer registers and automatic update of synchronous channels"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(UPDM_A::Mode1)
    }
    #[doc = "Automatic write of duty-cycle update registers by the PDC and automatic update of synchronous channels"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(UPDM_A::Mode2)
    }
}
#[doc = "Field `PTRM` reader - PDC Transfer Request Mode"]
pub type PTRM_R = crate::BitReader;
#[doc = "Field `PTRM` writer - PDC Transfer Request Mode"]
pub type PTRM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTRCS` reader - PDC Transfer Request Comparison Selection"]
pub type PTRCS_R = crate::FieldReader;
#[doc = "Field `PTRCS` writer - PDC Transfer Request Comparison Selection"]
pub type PTRCS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bit 0 - Synchronous Channel 0"]
    #[inline(always)]
    pub fn sync0(&self) -> SYNC0_R {
        SYNC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronous Channel 1"]
    #[inline(always)]
    pub fn sync1(&self) -> SYNC1_R {
        SYNC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronous Channel 2"]
    #[inline(always)]
    pub fn sync2(&self) -> SYNC2_R {
        SYNC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronous Channel 3"]
    #[inline(always)]
    pub fn sync3(&self) -> SYNC3_R {
        SYNC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Synchronous Channel 4"]
    #[inline(always)]
    pub fn sync4(&self) -> SYNC4_R {
        SYNC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Synchronous Channel 5"]
    #[inline(always)]
    pub fn sync5(&self) -> SYNC5_R {
        SYNC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Synchronous Channel 6"]
    #[inline(always)]
    pub fn sync6(&self) -> SYNC6_R {
        SYNC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronous Channel 7"]
    #[inline(always)]
    pub fn sync7(&self) -> SYNC7_R {
        SYNC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Synchronous Channels Update Mode"]
    #[inline(always)]
    pub fn updm(&self) -> UPDM_R {
        UPDM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - PDC Transfer Request Mode"]
    #[inline(always)]
    pub fn ptrm(&self) -> PTRM_R {
        PTRM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - PDC Transfer Request Comparison Selection"]
    #[inline(always)]
    pub fn ptrcs(&self) -> PTRCS_R {
        PTRCS_R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronous Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn sync0(&mut self) -> SYNC0_W<SCM_SPEC, 0> {
        SYNC0_W::new(self)
    }
    #[doc = "Bit 1 - Synchronous Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn sync1(&mut self) -> SYNC1_W<SCM_SPEC, 1> {
        SYNC1_W::new(self)
    }
    #[doc = "Bit 2 - Synchronous Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn sync2(&mut self) -> SYNC2_W<SCM_SPEC, 2> {
        SYNC2_W::new(self)
    }
    #[doc = "Bit 3 - Synchronous Channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn sync3(&mut self) -> SYNC3_W<SCM_SPEC, 3> {
        SYNC3_W::new(self)
    }
    #[doc = "Bit 4 - Synchronous Channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn sync4(&mut self) -> SYNC4_W<SCM_SPEC, 4> {
        SYNC4_W::new(self)
    }
    #[doc = "Bit 5 - Synchronous Channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn sync5(&mut self) -> SYNC5_W<SCM_SPEC, 5> {
        SYNC5_W::new(self)
    }
    #[doc = "Bit 6 - Synchronous Channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn sync6(&mut self) -> SYNC6_W<SCM_SPEC, 6> {
        SYNC6_W::new(self)
    }
    #[doc = "Bit 7 - Synchronous Channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn sync7(&mut self) -> SYNC7_W<SCM_SPEC, 7> {
        SYNC7_W::new(self)
    }
    #[doc = "Bits 16:17 - Synchronous Channels Update Mode"]
    #[inline(always)]
    #[must_use]
    pub fn updm(&mut self) -> UPDM_W<SCM_SPEC, 16> {
        UPDM_W::new(self)
    }
    #[doc = "Bit 20 - PDC Transfer Request Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ptrm(&mut self) -> PTRM_W<SCM_SPEC, 20> {
        PTRM_W::new(self)
    }
    #[doc = "Bits 21:23 - PDC Transfer Request Comparison Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ptrcs(&mut self) -> PTRCS_W<SCM_SPEC, 21> {
        PTRCS_W::new(self)
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
#[doc = "PWM Sync Channels Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCM_SPEC;
impl crate::RegisterSpec for SCM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scm::R`](R) reader structure"]
impl crate::Readable for SCM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scm::W`](W) writer structure"]
impl crate::Writable for SCM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCM to value 0"]
impl crate::Resettable for SCM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
