#[doc = "Register `SCM` reader"]
pub type R = crate::R<ScmSpec>;
#[doc = "Register `SCM` writer"]
pub type W = crate::W<ScmSpec>;
#[doc = "Field `SYNC0` reader - Synchronous Channel 0"]
pub type Sync0R = crate::BitReader;
#[doc = "Field `SYNC0` writer - Synchronous Channel 0"]
pub type Sync0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC1` reader - Synchronous Channel 1"]
pub type Sync1R = crate::BitReader;
#[doc = "Field `SYNC1` writer - Synchronous Channel 1"]
pub type Sync1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC2` reader - Synchronous Channel 2"]
pub type Sync2R = crate::BitReader;
#[doc = "Field `SYNC2` writer - Synchronous Channel 2"]
pub type Sync2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC3` reader - Synchronous Channel 3"]
pub type Sync3R = crate::BitReader;
#[doc = "Field `SYNC3` writer - Synchronous Channel 3"]
pub type Sync3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC4` reader - Synchronous Channel 4"]
pub type Sync4R = crate::BitReader;
#[doc = "Field `SYNC4` writer - Synchronous Channel 4"]
pub type Sync4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC5` reader - Synchronous Channel 5"]
pub type Sync5R = crate::BitReader;
#[doc = "Field `SYNC5` writer - Synchronous Channel 5"]
pub type Sync5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC6` reader - Synchronous Channel 6"]
pub type Sync6R = crate::BitReader;
#[doc = "Field `SYNC6` writer - Synchronous Channel 6"]
pub type Sync6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC7` reader - Synchronous Channel 7"]
pub type Sync7R = crate::BitReader;
#[doc = "Field `SYNC7` writer - Synchronous Channel 7"]
pub type Sync7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Synchronous Channels Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Updm {
    #[doc = "0: Manual write of double buffer registers and manual update of synchronous channels"]
    Mode0 = 0,
    #[doc = "1: Manual write of double buffer registers and automatic update of synchronous channels"]
    Mode1 = 1,
    #[doc = "2: Automatic write of duty-cycle update registers by the PDC and automatic update of synchronous channels"]
    Mode2 = 2,
}
impl From<Updm> for u8 {
    #[inline(always)]
    fn from(variant: Updm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Updm {
    type Ux = u8;
}
impl crate::IsEnum for Updm {}
#[doc = "Field `UPDM` reader - Synchronous Channels Update Mode"]
pub type UpdmR = crate::FieldReader<Updm>;
impl UpdmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Updm> {
        match self.bits {
            0 => Some(Updm::Mode0),
            1 => Some(Updm::Mode1),
            2 => Some(Updm::Mode2),
            _ => None,
        }
    }
    #[doc = "Manual write of double buffer registers and manual update of synchronous channels"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == Updm::Mode0
    }
    #[doc = "Manual write of double buffer registers and automatic update of synchronous channels"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == Updm::Mode1
    }
    #[doc = "Automatic write of duty-cycle update registers by the PDC and automatic update of synchronous channels"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == Updm::Mode2
    }
}
#[doc = "Field `UPDM` writer - Synchronous Channels Update Mode"]
pub type UpdmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Updm>;
impl<'a, REG> UpdmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Manual write of double buffer registers and manual update of synchronous channels"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(Updm::Mode0)
    }
    #[doc = "Manual write of double buffer registers and automatic update of synchronous channels"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(Updm::Mode1)
    }
    #[doc = "Automatic write of duty-cycle update registers by the PDC and automatic update of synchronous channels"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(Updm::Mode2)
    }
}
#[doc = "Field `PTRM` reader - PDC Transfer Request Mode"]
pub type PtrmR = crate::BitReader;
#[doc = "Field `PTRM` writer - PDC Transfer Request Mode"]
pub type PtrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTRCS` reader - PDC Transfer Request Comparison Selection"]
pub type PtrcsR = crate::FieldReader;
#[doc = "Field `PTRCS` writer - PDC Transfer Request Comparison Selection"]
pub type PtrcsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Synchronous Channel 0"]
    #[inline(always)]
    pub fn sync0(&self) -> Sync0R {
        Sync0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronous Channel 1"]
    #[inline(always)]
    pub fn sync1(&self) -> Sync1R {
        Sync1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronous Channel 2"]
    #[inline(always)]
    pub fn sync2(&self) -> Sync2R {
        Sync2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronous Channel 3"]
    #[inline(always)]
    pub fn sync3(&self) -> Sync3R {
        Sync3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Synchronous Channel 4"]
    #[inline(always)]
    pub fn sync4(&self) -> Sync4R {
        Sync4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Synchronous Channel 5"]
    #[inline(always)]
    pub fn sync5(&self) -> Sync5R {
        Sync5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Synchronous Channel 6"]
    #[inline(always)]
    pub fn sync6(&self) -> Sync6R {
        Sync6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronous Channel 7"]
    #[inline(always)]
    pub fn sync7(&self) -> Sync7R {
        Sync7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Synchronous Channels Update Mode"]
    #[inline(always)]
    pub fn updm(&self) -> UpdmR {
        UpdmR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - PDC Transfer Request Mode"]
    #[inline(always)]
    pub fn ptrm(&self) -> PtrmR {
        PtrmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - PDC Transfer Request Comparison Selection"]
    #[inline(always)]
    pub fn ptrcs(&self) -> PtrcsR {
        PtrcsR::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronous Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn sync0(&mut self) -> Sync0W<ScmSpec> {
        Sync0W::new(self, 0)
    }
    #[doc = "Bit 1 - Synchronous Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn sync1(&mut self) -> Sync1W<ScmSpec> {
        Sync1W::new(self, 1)
    }
    #[doc = "Bit 2 - Synchronous Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn sync2(&mut self) -> Sync2W<ScmSpec> {
        Sync2W::new(self, 2)
    }
    #[doc = "Bit 3 - Synchronous Channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn sync3(&mut self) -> Sync3W<ScmSpec> {
        Sync3W::new(self, 3)
    }
    #[doc = "Bit 4 - Synchronous Channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn sync4(&mut self) -> Sync4W<ScmSpec> {
        Sync4W::new(self, 4)
    }
    #[doc = "Bit 5 - Synchronous Channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn sync5(&mut self) -> Sync5W<ScmSpec> {
        Sync5W::new(self, 5)
    }
    #[doc = "Bit 6 - Synchronous Channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn sync6(&mut self) -> Sync6W<ScmSpec> {
        Sync6W::new(self, 6)
    }
    #[doc = "Bit 7 - Synchronous Channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn sync7(&mut self) -> Sync7W<ScmSpec> {
        Sync7W::new(self, 7)
    }
    #[doc = "Bits 16:17 - Synchronous Channels Update Mode"]
    #[inline(always)]
    #[must_use]
    pub fn updm(&mut self) -> UpdmW<ScmSpec> {
        UpdmW::new(self, 16)
    }
    #[doc = "Bit 20 - PDC Transfer Request Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ptrm(&mut self) -> PtrmW<ScmSpec> {
        PtrmW::new(self, 20)
    }
    #[doc = "Bits 21:23 - PDC Transfer Request Comparison Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ptrcs(&mut self) -> PtrcsW<ScmSpec> {
        PtrcsW::new(self, 21)
    }
}
#[doc = "PWM Sync Channels Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScmSpec;
impl crate::RegisterSpec for ScmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scm::R`](R) reader structure"]
impl crate::Readable for ScmSpec {}
#[doc = "`write(|w| ..)` method takes [`scm::W`](W) writer structure"]
impl crate::Writable for ScmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCM to value 0"]
impl crate::Resettable for ScmSpec {
    const RESET_VALUE: u32 = 0;
}
