#[doc = "Register `MMR7` reader"]
pub struct R(crate::R<MMR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMR7` writer"]
pub struct W(crate::W<MMR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMR7_SPEC>;
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
impl From<crate::W<MMR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMR7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MTIMEMARK` reader - Mailbox Timemark"]
pub type MTIMEMARK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MTIMEMARK` writer - Mailbox Timemark"]
pub type MTIMEMARK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMR7_SPEC, u16, u16, 16, O>;
#[doc = "Field `PRIOR` reader - Mailbox Priority"]
pub type PRIOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIOR` writer - Mailbox Priority"]
pub type PRIOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMR7_SPEC, u8, u8, 4, O>;
#[doc = "Field `MOT` reader - Mailbox Object Type"]
pub type MOT_R = crate::FieldReader<u8, MOT_A>;
#[doc = "Mailbox Object Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MOT_A {
    #[doc = "0: Mailbox is disabled. This prevents receiving or transmitting any messages with this mailbox."]
    MB_DISABLED = 0,
    #[doc = "1: Reception Mailbox. Mailbox is configured for reception. If a message is received while the mailbox data register is full, it is discarded."]
    MB_RX = 1,
    #[doc = "2: Reception mailbox with overwrite. Mailbox is configured for reception. If a message is received while the mailbox is full, it overwrites the previous message."]
    MB_RX_OVERWRITE = 2,
    #[doc = "3: Transmit mailbox. Mailbox is configured for transmission."]
    MB_TX = 3,
    #[doc = "4: Consumer Mailbox. Mailbox is configured in reception but behaves as a Transmit Mailbox, i.e., it sends a remote frame and waits for an answer."]
    MB_CONSUMER = 4,
    #[doc = "5: Producer Mailbox. Mailbox is configured in transmission but also behaves like a reception mailbox, i.e., it waits to receive a Remote Frame before sending its contents."]
    MB_PRODUCER = 5,
}
impl From<MOT_A> for u8 {
    #[inline(always)]
    fn from(variant: MOT_A) -> Self {
        variant as _
    }
}
impl MOT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MOT_A> {
        match self.bits {
            0 => Some(MOT_A::MB_DISABLED),
            1 => Some(MOT_A::MB_RX),
            2 => Some(MOT_A::MB_RX_OVERWRITE),
            3 => Some(MOT_A::MB_TX),
            4 => Some(MOT_A::MB_CONSUMER),
            5 => Some(MOT_A::MB_PRODUCER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MB_DISABLED`"]
    #[inline(always)]
    pub fn is_mb_disabled(&self) -> bool {
        *self == MOT_A::MB_DISABLED
    }
    #[doc = "Checks if the value of the field is `MB_RX`"]
    #[inline(always)]
    pub fn is_mb_rx(&self) -> bool {
        *self == MOT_A::MB_RX
    }
    #[doc = "Checks if the value of the field is `MB_RX_OVERWRITE`"]
    #[inline(always)]
    pub fn is_mb_rx_overwrite(&self) -> bool {
        *self == MOT_A::MB_RX_OVERWRITE
    }
    #[doc = "Checks if the value of the field is `MB_TX`"]
    #[inline(always)]
    pub fn is_mb_tx(&self) -> bool {
        *self == MOT_A::MB_TX
    }
    #[doc = "Checks if the value of the field is `MB_CONSUMER`"]
    #[inline(always)]
    pub fn is_mb_consumer(&self) -> bool {
        *self == MOT_A::MB_CONSUMER
    }
    #[doc = "Checks if the value of the field is `MB_PRODUCER`"]
    #[inline(always)]
    pub fn is_mb_producer(&self) -> bool {
        *self == MOT_A::MB_PRODUCER
    }
}
#[doc = "Field `MOT` writer - Mailbox Object Type"]
pub type MOT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMR7_SPEC, u8, MOT_A, 3, O>;
impl<'a, const O: u8> MOT_W<'a, O> {
    #[doc = "Mailbox is disabled. This prevents receiving or transmitting any messages with this mailbox."]
    #[inline(always)]
    pub fn mb_disabled(self) -> &'a mut W {
        self.variant(MOT_A::MB_DISABLED)
    }
    #[doc = "Reception Mailbox. Mailbox is configured for reception. If a message is received while the mailbox data register is full, it is discarded."]
    #[inline(always)]
    pub fn mb_rx(self) -> &'a mut W {
        self.variant(MOT_A::MB_RX)
    }
    #[doc = "Reception mailbox with overwrite. Mailbox is configured for reception. If a message is received while the mailbox is full, it overwrites the previous message."]
    #[inline(always)]
    pub fn mb_rx_overwrite(self) -> &'a mut W {
        self.variant(MOT_A::MB_RX_OVERWRITE)
    }
    #[doc = "Transmit mailbox. Mailbox is configured for transmission."]
    #[inline(always)]
    pub fn mb_tx(self) -> &'a mut W {
        self.variant(MOT_A::MB_TX)
    }
    #[doc = "Consumer Mailbox. Mailbox is configured in reception but behaves as a Transmit Mailbox, i.e., it sends a remote frame and waits for an answer."]
    #[inline(always)]
    pub fn mb_consumer(self) -> &'a mut W {
        self.variant(MOT_A::MB_CONSUMER)
    }
    #[doc = "Producer Mailbox. Mailbox is configured in transmission but also behaves like a reception mailbox, i.e., it waits to receive a Remote Frame before sending its contents."]
    #[inline(always)]
    pub fn mb_producer(self) -> &'a mut W {
        self.variant(MOT_A::MB_PRODUCER)
    }
}
impl R {
    #[doc = "Bits 0:15 - Mailbox Timemark"]
    #[inline(always)]
    pub fn mtimemark(&self) -> MTIMEMARK_R {
        MTIMEMARK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Mailbox Priority"]
    #[inline(always)]
    pub fn prior(&self) -> PRIOR_R {
        PRIOR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Mailbox Object Type"]
    #[inline(always)]
    pub fn mot(&self) -> MOT_R {
        MOT_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mailbox Timemark"]
    #[inline(always)]
    pub fn mtimemark(&mut self) -> MTIMEMARK_W<0> {
        MTIMEMARK_W::new(self)
    }
    #[doc = "Bits 16:19 - Mailbox Priority"]
    #[inline(always)]
    pub fn prior(&mut self) -> PRIOR_W<16> {
        PRIOR_W::new(self)
    }
    #[doc = "Bits 24:26 - Mailbox Object Type"]
    #[inline(always)]
    pub fn mot(&mut self) -> MOT_W<24> {
        MOT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mailbox Mode Register (MB = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmr7](index.html) module"]
pub struct MMR7_SPEC;
impl crate::RegisterSpec for MMR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmr7::R](R) reader structure"]
impl crate::Readable for MMR7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmr7::W](W) writer structure"]
impl crate::Writable for MMR7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMR7 to value 0"]
impl crate::Resettable for MMR7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
