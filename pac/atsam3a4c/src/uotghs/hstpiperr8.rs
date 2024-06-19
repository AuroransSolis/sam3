#[doc = "Register `HSTPIPERR8` reader"]
pub type R = crate::R<Hstpiperr8Spec>;
#[doc = "Register `HSTPIPERR8` writer"]
pub type W = crate::W<Hstpiperr8Spec>;
#[doc = "Field `DATATGL` reader - Data Toggle Error"]
pub type DatatglR = crate::BitReader;
#[doc = "Field `DATATGL` writer - Data Toggle Error"]
pub type DatatglW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAPID` reader - Data PID Error"]
pub type DatapidR = crate::BitReader;
#[doc = "Field `DATAPID` writer - Data PID Error"]
pub type DatapidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PID` reader - PID Error"]
pub type PidR = crate::BitReader;
#[doc = "Field `PID` writer - PID Error"]
pub type PidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - Time-Out Error"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - Time-Out Error"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC16` reader - CRC16 Error"]
pub type Crc16R = crate::BitReader;
#[doc = "Field `CRC16` writer - CRC16 Error"]
pub type Crc16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNTER` reader - Error Counter"]
pub type CounterR = crate::FieldReader;
#[doc = "Field `COUNTER` writer - Error Counter"]
pub type CounterW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Data Toggle Error"]
    #[inline(always)]
    pub fn datatgl(&self) -> DatatglR {
        DatatglR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data PID Error"]
    #[inline(always)]
    pub fn datapid(&self) -> DatapidR {
        DatapidR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PID Error"]
    #[inline(always)]
    pub fn pid(&self) -> PidR {
        PidR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time-Out Error"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC16 Error"]
    #[inline(always)]
    pub fn crc16(&self) -> Crc16R {
        Crc16R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Error Counter"]
    #[inline(always)]
    pub fn counter(&self) -> CounterR {
        CounterR::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Data Toggle Error"]
    #[inline(always)]
    #[must_use]
    pub fn datatgl(&mut self) -> DatatglW<Hstpiperr8Spec> {
        DatatglW::new(self, 0)
    }
    #[doc = "Bit 1 - Data PID Error"]
    #[inline(always)]
    #[must_use]
    pub fn datapid(&mut self) -> DatapidW<Hstpiperr8Spec> {
        DatapidW::new(self, 1)
    }
    #[doc = "Bit 2 - PID Error"]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PidW<Hstpiperr8Spec> {
        PidW::new(self, 2)
    }
    #[doc = "Bit 3 - Time-Out Error"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<Hstpiperr8Spec> {
        TimeoutW::new(self, 3)
    }
    #[doc = "Bit 4 - CRC16 Error"]
    #[inline(always)]
    #[must_use]
    pub fn crc16(&mut self) -> Crc16W<Hstpiperr8Spec> {
        Crc16W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Error Counter"]
    #[inline(always)]
    #[must_use]
    pub fn counter(&mut self) -> CounterW<Hstpiperr8Spec> {
        CounterW::new(self, 5)
    }
}
#[doc = "Host Pipe Error Register (n = 0) 8\n\nYou can [`read`](crate::Reg::read) this register and get [`hstpiperr8::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstpiperr8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hstpiperr8Spec;
impl crate::RegisterSpec for Hstpiperr8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstpiperr8::R`](R) reader structure"]
impl crate::Readable for Hstpiperr8Spec {}
#[doc = "`write(|w| ..)` method takes [`hstpiperr8::W`](W) writer structure"]
impl crate::Writable for Hstpiperr8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
