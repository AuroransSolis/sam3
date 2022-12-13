#[doc = "Register `MMR5` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<MMR5_SPEC>);
#[doc = "Register `MMR5` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<MMR5_SPEC>);
#[doc = "Field `MTIMEMARK` reader - Mailbox Timemark"]
pub type MTIMEMARK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MTIMEMARK` writer - Mailbox Timemark"]
pub type MTIMEMARK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMR5_SPEC, u16, u16, 16, O>;
#[doc = "Field `PRIOR` reader - Mailbox Priority"]
pub type PRIOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIOR` writer - Mailbox Priority"]
pub type PRIOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMR5_SPEC, u8, u8, 4, O>;
#[doc = "Field `MOT` reader - Mailbox Object Type"]
pub type MOT_R = crate::FieldReader<u8, MOT_A>;
#[doc = "Mailbox Object Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MOT_A {
    #[doc = "0: Mailbox is disabled. This prevents receiving or transmitting any messages with this mailbox."]
    MbDisabled = 0,
    #[doc = "1: Reception Mailbox. Mailbox is configured for reception. If a message is received while the mailbox data register is full, it is discarded."]
    MbRx = 1,
    #[doc = "2: Reception mailbox with overwrite. Mailbox is configured for reception. If a message is received while the mailbox is full, it overwrites the previous message."]
    MbRxOverwrite = 2,
    #[doc = "3: Transmit mailbox. Mailbox is configured for transmission."]
    MbTx = 3,
    #[doc = "4: Consumer Mailbox. Mailbox is configured in reception but behaves as a Transmit Mailbox, i.e., it sends a remote frame and waits for an answer."]
    MbConsumer = 4,
    #[doc = "5: Producer Mailbox. Mailbox is configured in transmission but also behaves like a reception mailbox, i.e., it waits to receive a Remote Frame before sending its contents."]
    MbProducer = 5,
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
            0 => Some(MOT_A::MbDisabled),
            1 => Some(MOT_A::MbRx),
            2 => Some(MOT_A::MbRxOverwrite),
            3 => Some(MOT_A::MbTx),
            4 => Some(MOT_A::MbConsumer),
            5 => Some(MOT_A::MbProducer),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MbDisabled`"]
    #[inline(always)]
    pub fn is_mb_disabled(&self) -> bool {
        *self == MOT_A::MbDisabled
    }
    #[doc = "Checks if the value of the field is `MbRx`"]
    #[inline(always)]
    pub fn is_mb_rx(&self) -> bool {
        *self == MOT_A::MbRx
    }
    #[doc = "Checks if the value of the field is `MbRxOverwrite`"]
    #[inline(always)]
    pub fn is_mb_rx_overwrite(&self) -> bool {
        *self == MOT_A::MbRxOverwrite
    }
    #[doc = "Checks if the value of the field is `MbTx`"]
    #[inline(always)]
    pub fn is_mb_tx(&self) -> bool {
        *self == MOT_A::MbTx
    }
    #[doc = "Checks if the value of the field is `MbConsumer`"]
    #[inline(always)]
    pub fn is_mb_consumer(&self) -> bool {
        *self == MOT_A::MbConsumer
    }
    #[doc = "Checks if the value of the field is `MbProducer`"]
    #[inline(always)]
    pub fn is_mb_producer(&self) -> bool {
        *self == MOT_A::MbProducer
    }
}
#[doc = "Field `MOT` writer - Mailbox Object Type"]
pub type MOT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMR5_SPEC, u8, MOT_A, 3, O>;
impl<'a, const O: u8> MOT_W<'a, O> {
    #[doc = "Mailbox is disabled. This prevents receiving or transmitting any messages with this mailbox."]
    #[inline(always)]
    pub fn mb_disabled(self) -> &'a mut W {
        self.variant(MOT_A::MbDisabled)
    }
    #[doc = "Reception Mailbox. Mailbox is configured for reception. If a message is received while the mailbox data register is full, it is discarded."]
    #[inline(always)]
    pub fn mb_rx(self) -> &'a mut W {
        self.variant(MOT_A::MbRx)
    }
    #[doc = "Reception mailbox with overwrite. Mailbox is configured for reception. If a message is received while the mailbox is full, it overwrites the previous message."]
    #[inline(always)]
    pub fn mb_rx_overwrite(self) -> &'a mut W {
        self.variant(MOT_A::MbRxOverwrite)
    }
    #[doc = "Transmit mailbox. Mailbox is configured for transmission."]
    #[inline(always)]
    pub fn mb_tx(self) -> &'a mut W {
        self.variant(MOT_A::MbTx)
    }
    #[doc = "Consumer Mailbox. Mailbox is configured in reception but behaves as a Transmit Mailbox, i.e., it sends a remote frame and waits for an answer."]
    #[inline(always)]
    pub fn mb_consumer(self) -> &'a mut W {
        self.variant(MOT_A::MbConsumer)
    }
    #[doc = "Producer Mailbox. Mailbox is configured in transmission but also behaves like a reception mailbox, i.e., it waits to receive a Remote Frame before sending its contents."]
    #[inline(always)]
    pub fn mb_producer(self) -> &'a mut W {
        self.variant(MOT_A::MbProducer)
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
    #[must_use]
    pub fn mtimemark(&mut self) -> MTIMEMARK_W<0> {
        MTIMEMARK_W::new(self)
    }
    #[doc = "Bits 16:19 - Mailbox Priority"]
    #[inline(always)]
    #[must_use]
    pub fn prior(&mut self) -> PRIOR_W<16> {
        PRIOR_W::new(self)
    }
    #[doc = "Bits 24:26 - Mailbox Object Type"]
    #[inline(always)]
    #[must_use]
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
#[doc = "Mailbox Mode Register (MB = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmr5](index.html) module"]
pub struct MMR5_SPEC;
impl crate::RegisterSpec for MMR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmr5::R](R) reader structure"]
impl crate::Readable for MMR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmr5::W](W) writer structure"]
impl crate::Writable for MMR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMR5 to value 0"]
impl crate::Resettable for MMR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
