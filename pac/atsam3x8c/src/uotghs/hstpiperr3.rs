#[doc = "Register `HSTPIPERR3` reader"]
pub type R = crate::R<HSTPIPERR3_SPEC>;
#[doc = "Register `HSTPIPERR3` writer"]
pub type W = crate::W<HSTPIPERR3_SPEC>;
#[doc = "Field `DATATGL` reader - Data Toggle Error"]
pub type DATATGL_R = crate::BitReader;
#[doc = "Field `DATATGL` writer - Data Toggle Error"]
pub type DATATGL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAPID` reader - Data PID Error"]
pub type DATAPID_R = crate::BitReader;
#[doc = "Field `DATAPID` writer - Data PID Error"]
pub type DATAPID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID` reader - PID Error"]
pub type PID_R = crate::BitReader;
#[doc = "Field `PID` writer - PID Error"]
pub type PID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - Time-Out Error"]
pub type TIMEOUT_R = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - Time-Out Error"]
pub type TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC16` reader - CRC16 Error"]
pub type CRC16_R = crate::BitReader;
#[doc = "Field `CRC16` writer - CRC16 Error"]
pub type CRC16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNTER` reader - Error Counter"]
pub type COUNTER_R = crate::FieldReader;
#[doc = "Field `COUNTER` writer - Error Counter"]
pub type COUNTER_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Data Toggle Error"]
    #[inline(always)]
    pub fn datatgl(&self) -> DATATGL_R {
        DATATGL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data PID Error"]
    #[inline(always)]
    pub fn datapid(&self) -> DATAPID_R {
        DATAPID_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PID Error"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time-Out Error"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC16 Error"]
    #[inline(always)]
    pub fn crc16(&self) -> CRC16_R {
        CRC16_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Error Counter"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Data Toggle Error"]
    #[inline(always)]
    #[must_use]
    pub fn datatgl(&mut self) -> DATATGL_W<HSTPIPERR3_SPEC> {
        DATATGL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data PID Error"]
    #[inline(always)]
    #[must_use]
    pub fn datapid(&mut self) -> DATAPID_W<HSTPIPERR3_SPEC> {
        DATAPID_W::new(self, 1)
    }
    #[doc = "Bit 2 - PID Error"]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PID_W<HSTPIPERR3_SPEC> {
        PID_W::new(self, 2)
    }
    #[doc = "Bit 3 - Time-Out Error"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<HSTPIPERR3_SPEC> {
        TIMEOUT_W::new(self, 3)
    }
    #[doc = "Bit 4 - CRC16 Error"]
    #[inline(always)]
    #[must_use]
    pub fn crc16(&mut self) -> CRC16_W<HSTPIPERR3_SPEC> {
        CRC16_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Error Counter"]
    #[inline(always)]
    #[must_use]
    pub fn counter(&mut self) -> COUNTER_W<HSTPIPERR3_SPEC> {
        COUNTER_W::new(self, 5)
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
#[doc = "Host Pipe Error Register (n = 0) 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpiperr3::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpiperr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTPIPERR3_SPEC;
impl crate::RegisterSpec for HSTPIPERR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstpiperr3::R`](R) reader structure"]
impl crate::Readable for HSTPIPERR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hstpiperr3::W`](W) writer structure"]
impl crate::Writable for HSTPIPERR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
