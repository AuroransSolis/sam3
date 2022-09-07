#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEV_ADDR` reader - UDPHS Address"]
pub type DEV_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEV_ADDR` writer - UDPHS Address"]
pub type DEV_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 7, O>;
#[doc = "Field `FADDR_EN` reader - Function Address Enable"]
pub type FADDR_EN_R = crate::BitReader<bool>;
#[doc = "Field `FADDR_EN` writer - Function Address Enable"]
pub type FADDR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `EN_UDPHS` reader - UDPHS Enable"]
pub type EN_UDPHS_R = crate::BitReader<bool>;
#[doc = "Field `EN_UDPHS` writer - UDPHS Enable"]
pub type EN_UDPHS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DETACH` reader - Detach Command"]
pub type DETACH_R = crate::BitReader<bool>;
#[doc = "Field `DETACH` writer - Detach Command"]
pub type DETACH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `REWAKEUP` reader - Send Remote Wake Up"]
pub type REWAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `REWAKEUP` writer - Send Remote Wake Up"]
pub type REWAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PULLD_DIS` reader - Pull-Down Disable"]
pub type PULLD_DIS_R = crate::BitReader<bool>;
#[doc = "Field `PULLD_DIS` writer - Pull-Down Disable"]
pub type PULLD_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - UDPHS Address"]
    #[inline(always)]
    pub fn dev_addr(&self) -> DEV_ADDR_R {
        DEV_ADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Function Address Enable"]
    #[inline(always)]
    pub fn faddr_en(&self) -> FADDR_EN_R {
        FADDR_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UDPHS Enable"]
    #[inline(always)]
    pub fn en_udphs(&self) -> EN_UDPHS_R {
        EN_UDPHS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Detach Command"]
    #[inline(always)]
    pub fn detach(&self) -> DETACH_R {
        DETACH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Send Remote Wake Up"]
    #[inline(always)]
    pub fn rewakeup(&self) -> REWAKEUP_R {
        REWAKEUP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pull-Down Disable"]
    #[inline(always)]
    pub fn pulld_dis(&self) -> PULLD_DIS_R {
        PULLD_DIS_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - UDPHS Address"]
    #[inline(always)]
    pub fn dev_addr(&mut self) -> DEV_ADDR_W<0> {
        DEV_ADDR_W::new(self)
    }
    #[doc = "Bit 7 - Function Address Enable"]
    #[inline(always)]
    pub fn faddr_en(&mut self) -> FADDR_EN_W<7> {
        FADDR_EN_W::new(self)
    }
    #[doc = "Bit 8 - UDPHS Enable"]
    #[inline(always)]
    pub fn en_udphs(&mut self) -> EN_UDPHS_W<8> {
        EN_UDPHS_W::new(self)
    }
    #[doc = "Bit 9 - Detach Command"]
    #[inline(always)]
    pub fn detach(&mut self) -> DETACH_W<9> {
        DETACH_W::new(self)
    }
    #[doc = "Bit 10 - Send Remote Wake Up"]
    #[inline(always)]
    pub fn rewakeup(&mut self) -> REWAKEUP_W<10> {
        REWAKEUP_W::new(self)
    }
    #[doc = "Bit 11 - Pull-Down Disable"]
    #[inline(always)]
    pub fn pulld_dis(&mut self) -> PULLD_DIS_W<11> {
        PULLD_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UDPHS Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x0200"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
