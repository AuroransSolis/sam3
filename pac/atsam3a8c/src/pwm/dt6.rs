#[doc = "Register `DT6` reader"]
pub struct R(crate::R<DT6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DT6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DT6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DT6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DT6` writer"]
pub struct W(crate::W<DT6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DT6_SPEC>;
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
impl From<crate::W<DT6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DT6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTH` reader - Dead-Time Value for PWMHx Output"]
pub type DTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DTH` writer - Dead-Time Value for PWMHx Output"]
pub type DTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DT6_SPEC, u16, u16, 16, O>;
#[doc = "Field `DTL` reader - Dead-Time Value for PWMLx Output"]
pub type DTL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DTL` writer - Dead-Time Value for PWMLx Output"]
pub type DTL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DT6_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Dead-Time Value for PWMHx Output"]
    #[inline(always)]
    pub fn dth(&self) -> DTH_R {
        DTH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Dead-Time Value for PWMLx Output"]
    #[inline(always)]
    pub fn dtl(&self) -> DTL_R {
        DTL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Dead-Time Value for PWMHx Output"]
    #[inline(always)]
    pub fn dth(&mut self) -> DTH_W<0> {
        DTH_W::new(self)
    }
    #[doc = "Bits 16:31 - Dead-Time Value for PWMLx Output"]
    #[inline(always)]
    pub fn dtl(&mut self) -> DTL_W<16> {
        DTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Dead Time Register (ch_num = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dt6](index.html) module"]
pub struct DT6_SPEC;
impl crate::RegisterSpec for DT6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dt6::R](R) reader structure"]
impl crate::Readable for DT6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dt6::W](W) writer structure"]
impl crate::Writable for DT6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DT6 to value 0"]
impl crate::Resettable for DT6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
