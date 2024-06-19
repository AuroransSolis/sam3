#[doc = "Register `SMMR` reader"]
pub type R = crate::R<SmmrSpec>;
#[doc = "Register `SMMR` writer"]
pub type W = crate::W<SmmrSpec>;
#[doc = "Field `SMTH` reader - Supply Monitor Threshold"]
pub type SmthR = crate::FieldReader;
#[doc = "Field `SMTH` writer - Supply Monitor Threshold"]
pub type SmthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Supply Monitor Sampling Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Smsmpl {
    #[doc = "0: Supply Monitor disabled"]
    Smd = 0,
    #[doc = "1: Continuous Supply Monitor"]
    Csm = 1,
    #[doc = "2: Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    _32slck = 2,
    #[doc = "3: Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    _256slck = 3,
    #[doc = "4: Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    _2048slck = 4,
}
impl From<Smsmpl> for u8 {
    #[inline(always)]
    fn from(variant: Smsmpl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Smsmpl {
    type Ux = u8;
}
impl crate::IsEnum for Smsmpl {}
#[doc = "Field `SMSMPL` reader - Supply Monitor Sampling Period"]
pub type SmsmplR = crate::FieldReader<Smsmpl>;
impl SmsmplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Smsmpl> {
        match self.bits {
            0 => Some(Smsmpl::Smd),
            1 => Some(Smsmpl::Csm),
            2 => Some(Smsmpl::_32slck),
            3 => Some(Smsmpl::_256slck),
            4 => Some(Smsmpl::_2048slck),
            _ => None,
        }
    }
    #[doc = "Supply Monitor disabled"]
    #[inline(always)]
    pub fn is_smd(&self) -> bool {
        *self == Smsmpl::Smd
    }
    #[doc = "Continuous Supply Monitor"]
    #[inline(always)]
    pub fn is_csm(&self) -> bool {
        *self == Smsmpl::Csm
    }
    #[doc = "Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    #[inline(always)]
    pub fn is_32slck(&self) -> bool {
        *self == Smsmpl::_32slck
    }
    #[doc = "Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    #[inline(always)]
    pub fn is_256slck(&self) -> bool {
        *self == Smsmpl::_256slck
    }
    #[doc = "Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    #[inline(always)]
    pub fn is_2048slck(&self) -> bool {
        *self == Smsmpl::_2048slck
    }
}
#[doc = "Field `SMSMPL` writer - Supply Monitor Sampling Period"]
pub type SmsmplW<'a, REG> = crate::FieldWriter<'a, REG, 3, Smsmpl>;
impl<'a, REG> SmsmplW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Supply Monitor disabled"]
    #[inline(always)]
    pub fn smd(self) -> &'a mut crate::W<REG> {
        self.variant(Smsmpl::Smd)
    }
    #[doc = "Continuous Supply Monitor"]
    #[inline(always)]
    pub fn csm(self) -> &'a mut crate::W<REG> {
        self.variant(Smsmpl::Csm)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    #[inline(always)]
    pub fn _32slck(self) -> &'a mut crate::W<REG> {
        self.variant(Smsmpl::_32slck)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    #[inline(always)]
    pub fn _256slck(self) -> &'a mut crate::W<REG> {
        self.variant(Smsmpl::_256slck)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    #[inline(always)]
    pub fn _2048slck(self) -> &'a mut crate::W<REG> {
        self.variant(Smsmpl::_2048slck)
    }
}
#[doc = "Supply Monitor Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smrsten {
    #[doc = "0: the core reset signal \"vddcore_nreset\" is not affected when a supply monitor detection occurs."]
    NotEnable = 0,
    #[doc = "1: the core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    Enable = 1,
}
impl From<Smrsten> for bool {
    #[inline(always)]
    fn from(variant: Smrsten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMRSTEN` reader - Supply Monitor Reset Enable"]
pub type SmrstenR = crate::BitReader<Smrsten>;
impl SmrstenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smrsten {
        match self.bits {
            false => Smrsten::NotEnable,
            true => Smrsten::Enable,
        }
    }
    #[doc = "the core reset signal \"vddcore_nreset\" is not affected when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Smrsten::NotEnable
    }
    #[doc = "the core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Smrsten::Enable
    }
}
#[doc = "Field `SMRSTEN` writer - Supply Monitor Reset Enable"]
pub type SmrstenW<'a, REG> = crate::BitWriter<'a, REG, Smrsten>;
impl<'a, REG> SmrstenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the core reset signal \"vddcore_nreset\" is not affected when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Smrsten::NotEnable)
    }
    #[doc = "the core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Smrsten::Enable)
    }
}
#[doc = "Supply Monitor Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smien {
    #[doc = "0: the SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    NotEnable = 0,
    #[doc = "1: the SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    Enable = 1,
}
impl From<Smien> for bool {
    #[inline(always)]
    fn from(variant: Smien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMIEN` reader - Supply Monitor Interrupt Enable"]
pub type SmienR = crate::BitReader<Smien>;
impl SmienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smien {
        match self.bits {
            false => Smien::NotEnable,
            true => Smien::Enable,
        }
    }
    #[doc = "the SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Smien::NotEnable
    }
    #[doc = "the SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Smien::Enable
    }
}
#[doc = "Field `SMIEN` writer - Supply Monitor Interrupt Enable"]
pub type SmienW<'a, REG> = crate::BitWriter<'a, REG, Smien>;
impl<'a, REG> SmienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Smien::NotEnable)
    }
    #[doc = "the SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Smien::Enable)
    }
}
impl R {
    #[doc = "Bits 0:3 - Supply Monitor Threshold"]
    #[inline(always)]
    pub fn smth(&self) -> SmthR {
        SmthR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Supply Monitor Sampling Period"]
    #[inline(always)]
    pub fn smsmpl(&self) -> SmsmplR {
        SmsmplR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Supply Monitor Reset Enable"]
    #[inline(always)]
    pub fn smrsten(&self) -> SmrstenR {
        SmrstenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Supply Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn smien(&self) -> SmienR {
        SmienR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Supply Monitor Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn smth(&mut self) -> SmthW<SmmrSpec> {
        SmthW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Supply Monitor Sampling Period"]
    #[inline(always)]
    #[must_use]
    pub fn smsmpl(&mut self) -> SmsmplW<SmmrSpec> {
        SmsmplW::new(self, 8)
    }
    #[doc = "Bit 12 - Supply Monitor Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smrsten(&mut self) -> SmrstenW<SmmrSpec> {
        SmrstenW::new(self, 12)
    }
    #[doc = "Bit 13 - Supply Monitor Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smien(&mut self) -> SmienW<SmmrSpec> {
        SmienW::new(self, 13)
    }
}
#[doc = "Supply Controller Supply Monitor Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmmrSpec;
impl crate::RegisterSpec for SmmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smmr::R`](R) reader structure"]
impl crate::Readable for SmmrSpec {}
#[doc = "`write(|w| ..)` method takes [`smmr::W`](W) writer structure"]
impl crate::Writable for SmmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMMR to value 0"]
impl crate::Resettable for SmmrSpec {
    const RESET_VALUE: u32 = 0;
}
