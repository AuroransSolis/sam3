#[doc = "Register `DT6` reader"]
pub type R = crate::R<DT6_SPEC>;
#[doc = "Register `DT6` writer"]
pub type W = crate::W<DT6_SPEC>;
#[doc = "Field `DTH` reader - Dead-Time Value for PWMHx Output"]
pub type DTH_R = crate::FieldReader<u16>;
#[doc = "Field `DTH` writer - Dead-Time Value for PWMHx Output"]
pub type DTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `DTL` reader - Dead-Time Value for PWMLx Output"]
pub type DTL_R = crate::FieldReader<u16>;
#[doc = "Field `DTL` writer - Dead-Time Value for PWMLx Output"]
pub type DTL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
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
    #[must_use]
    pub fn dth(&mut self) -> DTH_W<DT6_SPEC, 0> {
        DTH_W::new(self)
    }
    #[doc = "Bits 16:31 - Dead-Time Value for PWMLx Output"]
    #[inline(always)]
    #[must_use]
    pub fn dtl(&mut self) -> DTL_W<DT6_SPEC, 16> {
        DTL_W::new(self)
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
#[doc = "PWM Channel Dead Time Register (ch_num = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT6_SPEC;
impl crate::RegisterSpec for DT6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt6::R`](R) reader structure"]
impl crate::Readable for DT6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt6::W`](W) writer structure"]
impl crate::Writable for DT6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT6 to value 0"]
impl crate::Resettable for DT6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
